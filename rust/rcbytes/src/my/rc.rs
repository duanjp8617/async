use std::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::alloc::{alloc, dealloc, Layout};
use std::ops::{Deref};

struct RcBox<T> {
    strong : Cell<usize>,
    weak : Cell<usize>,
    value : T
}

pub struct Rc<T> {
    ptr : NonNull<RcBox<T>>,
    _data : PhantomData<T>
}

impl<T> Rc<T> {
    pub fn new(value : T) -> Self {
        unsafe {
            let ptr = alloc(Layout::new::<RcBox<T>>()) as *mut RcBox<T>;
            std::ptr::write(ptr, RcBox{
                strong : Cell::new(1),
                weak : Cell::new(1),
                value : value,
            });
            Rc {
                ptr : NonNull::new_unchecked(ptr),
                _data : PhantomData,
            }
        }
    }

    pub fn downgrade(this : &Self) -> Weak<T> {
        this.inc_weak();
        Weak {
            ptr : this.ptr
        }
    }

    fn inner(&self) -> &RcBox<T> {
        unsafe{self.ptr.as_ref()}
    }

    fn dec_strong(&self) {        
        self.inner().strong.set(self.inner().strong.get()-1);
    }

    fn inc_strong(&self) {
        self.inner().strong.set(self.inner().strong.get()+1);
    }

    fn strong_count(&self) -> usize {    
        self.inner().strong.get()
    }

    fn dec_weak(&self) {
        self.inner().weak.set(self.inner().weak.get()-1);
    }

    fn inc_weak(&self) {
        self.inner().weak.set(self.inner().weak.get()+1);
    }

    fn weak_count(&self) -> usize {
        self.inner().weak.get()
    }

}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        self.dec_strong();
        
        if self.strong_count() == 0 {
            unsafe{std::ptr::drop_in_place(self.ptr.as_mut())};

            self.dec_weak();

            if self.weak_count() == 0 {
                println!("Strong deallocating RcBox");
                unsafe {dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<RcBox<T>>())};
            }
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        & self.inner().value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        self.inc_strong();
        Rc {
            ptr : self.ptr,
            _data : PhantomData,
        }
    }
}

pub struct Weak<T> {
    ptr : NonNull<RcBox<T>>,
}

impl<T> Weak<T> {
    pub fn upgrade(&self) -> Option<Rc<T>> {
        let strong_count = self.inner().strong.get();
        if strong_count > 0 {
            self.inner().strong.set(strong_count + 1);
            Some(Rc {
                ptr : self.ptr,
                _data : PhantomData,
            })
        }
        else {
            None
        }
    }

    fn inner(&self) -> &RcBox<T> {
        unsafe{self.ptr.as_ref()}
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        let weak_count = self.inner().weak.get();
        if weak_count == 1 {
            println!("Weak deallocating RcBox");
            unsafe{dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<RcBox<T>>())}
        }
        else {
            self.inner().weak.set(weak_count - 1);
        }
    }
}