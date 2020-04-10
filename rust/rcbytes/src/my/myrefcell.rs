use std::mem;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct MyUnsafeCell <T> {
    value : T,
}

impl<T> MyUnsafeCell<T> {
    pub fn new(value : T) -> Self{
        MyUnsafeCell {
            value : value
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn get(&self) -> *mut T {
        self as *const MyUnsafeCell<T> as *const T as *mut T
    }
}

#[repr(transparent)]
pub struct MyCell<T> {
    inner : MyUnsafeCell<T>
}

impl<T> MyCell<T> {
    pub fn new(value : T) -> Self {
        MyCell {
            inner : MyUnsafeCell::new(value)
        }
    }

    pub fn set(&self, new_value : T) {
        let old_value = self.replace(new_value);
        drop(old_value)
    }

    pub fn swap(&self, other : &Self) {
        if self.inner.get() != other.inner.get() {
            unsafe {mem::swap(&mut (*self.inner.get()), &mut (*other.inner.get()))}
        } 
    }

    pub fn replace(&self, new_value : T) -> T {
        unsafe {mem::replace(&mut (*self.inner.get()), new_value)}
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T> MyCell<T> {
    pub unsafe fn as_ptr(&self) -> *mut T {
        self.inner.get()
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe{&mut *self.inner.get()}
    }
}

impl<T: Copy> MyCell<T> {
    pub fn get(&self) -> T {
        self.inner.value
    }

    pub fn update<F>(&self, f : F) -> T
    where 
        F : FnOnce(T) -> T 
    {
        let old_value = self.get();
        self.set(f(old_value));
        old_value
    }
}

pub struct MyRefCell<T> {
    value : MyUnsafeCell<T>,
    borrow_counter : MyCell<isize>,
}

impl<T> MyRefCell<T> {
    pub fn new(value : T) -> Self {
        Self{
            value : MyUnsafeCell::new(value),
            borrow_counter : MyCell::new(0),
        }
    }

    // have to explicitly specify the lifetime here, otherwise rust compiler 
    // complains badly
    pub fn try_borrow<'b>(&'b self) -> Result<MyRef<'b, T>, MyBorrowError> {
        match MyBorrowRef::try_borrow(&self.borrow_counter) {
            Some(br) => {
                Ok(MyRef {
                    // & unsafe{*self.value.get()} cases a move due to the added scope
                    value_ref : unsafe {& *self.value.get()}, 
                    borrow_ref : br,
                })
            }
            None => {
                Err(MyBorrowError {
                    _private: ()
                })
            }
        }
    }

    pub fn try_borrow_mut<'b>(&'b self) -> Result<MyRefMut<'b, T>, MyBorrowError> {
        match MyBorrowRefMut::try_borrow(&self.borrow_counter) {
            Some(br) => {
                Ok(MyRefMut {
                    // & unsafe{*self.value.get()} cases a move due to the added scope
                    value_mut_ref : unsafe {&mut *self.value.get()},
                    borrow_mut_ref : br,
                })
            }
            None => {
                Err(MyBorrowError {
                    _private: ()
                })
            }
        }
    }

    pub fn borrow<'b>(&'b self) -> MyRef<'b, T> {
        self.try_borrow().expect("can not borrow as MyRef")
    }

    pub fn borrow_mut<'b>(&'b self) -> MyRefMut<'b, T> {
        self.try_borrow_mut().expect("can not borrow as MyRefMut")
    }
}

struct MyBorrowRef<'b> {
    borrow_counter_ref : &'b MyCell<isize>, 
}

impl<'b> MyBorrowRef<'b> {
    fn try_borrow(counter : &'b MyCell<isize>) -> Option<Self> {
        let borrow_counter = counter.get();
        if borrow_counter < 0 {
            // MyRefCell has been mutably borrowed
            None
        }
        else {
            counter.set(borrow_counter+1);
            Some(Self {
                borrow_counter_ref : counter
            })
        }
    }
}

impl<'b> Drop for MyBorrowRef<'b> {
    fn drop(&mut self) {
        let borrow_counter = self.borrow_counter_ref.get();
        assert!(borrow_counter > 0);
        self.borrow_counter_ref.set(borrow_counter-1);
    }
}

#[allow(dead_code)]
pub struct MyRef<'b, T: 'b> {
    value_ref : &'b T,
    borrow_ref : MyBorrowRef<'b>,
}

impl<'b, T> Deref for MyRef<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value_ref
    }
}

impl<'b, T> MyRef<'b, T> {
    pub fn map<F, U>(self, f : F) -> MyRef<'b, U> 
    where F : Fn(&T) -> &U {
        MyRef {
            value_ref : f(self.value_ref),
            borrow_ref : self.borrow_ref
        }
    }
}

struct MyBorrowRefMut<'b> {
    borrow_counter_ref : &'b MyCell<isize>,
}

impl<'b> MyBorrowRefMut<'b> {
    fn try_borrow(counter : &'b MyCell<isize>) -> Option<Self> {
        let borrow_counter = counter.get();
        if borrow_counter != 0 {
            // MyRefCell has been mutably borrowed
            None
        }
        else {
            counter.set(borrow_counter-1);
            Some(Self {
                borrow_counter_ref : counter
            })
        }
    }
}

impl<'b> Drop for MyBorrowRefMut<'b> {
    fn drop(&mut self) {
        let borrow_counter = self.borrow_counter_ref.get();
        assert!(borrow_counter == -1);
        self.borrow_counter_ref.set(0);
    }
}

#[allow(dead_code)]
pub struct MyRefMut<'b, T: 'b> {
    value_mut_ref : &'b mut T,
    borrow_mut_ref : MyBorrowRefMut<'b>,
}

impl<'b, T> Deref for MyRefMut<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value_mut_ref
    }
}

impl<'b, T> MyRefMut<'b, T> {
    pub fn map<F, U>(self, mut f : F) -> MyRefMut<'b, U> 
    where F : FnMut(&mut T) -> &mut U {
        MyRefMut {
            value_mut_ref : f(self.value_mut_ref),
            borrow_mut_ref : self.borrow_mut_ref,
        }
    }
}

impl<'b, T> DerefMut for MyRefMut<'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut_ref
    }
}

#[derive(Debug)]
pub struct MyBorrowError {
    _private: (),
}