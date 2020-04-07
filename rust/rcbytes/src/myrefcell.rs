use std::mem;

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