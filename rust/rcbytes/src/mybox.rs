use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{Unique, self};
use std::ops::{Deref, DerefMut};

pub struct MyBox<T> {
    ptr : Unique<T>,
}

impl<T> MyBox<T> {
    pub fn new(x : T) -> Self {
        assert!(std::mem::size_of::<T>() != 0);
        let ptr = unsafe{alloc(Layout::new::<T>()) as *mut T};
        unsafe {
            ptr::write(ptr, x);
        }
        MyBox {
            ptr : unsafe {Unique::new_unchecked(ptr)},
        }
    }

    pub unsafe fn from_raw(x : *mut T) -> Self {
        MyBox {
            ptr : Unique::new_unchecked(x),
        }
    }

    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr.as_ptr();
        std::mem::forget(self);
        ptr
    }

    pub fn leak<'a>(self) -> &'a mut T where T : 'a{
        unsafe { &mut *self.into_raw() }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        let ptr = self.ptr.as_ptr();
        unsafe {
            std::ptr::read(ptr);
            dealloc(ptr as *mut u8, Layout::new::<T>());
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {self.ptr.as_ref()}
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {self.ptr.as_mut()}
    }
}