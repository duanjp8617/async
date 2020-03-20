use std::marker::PhantomData;
use std::mem;
use std::cmp;
use std::ptr;

use crate::myvec::MyVec;


pub struct MySlice<'a , T : 'a> {
    ptr : *const T,
    len : usize,
    data : PhantomData<&'a T>
}

impl<'a, T> MySlice<'a, T> {
    pub fn from(vec : &'a MyVec<T>) -> Self {
        MySlice {
            ptr : vec.ptr.as_ptr(),
            len : vec.len,
            data : PhantomData,
        }
    }

    pub fn at(&self, index : usize) -> Option<&T> {
        if self.len == 0 || index >= self.len {
            None
        }
        else {
            unsafe {
                Some(&(*self.ptr.add(index)))
            }
        }
    }
}

impl<'a, T> Clone for MySlice<'a, T> {
    fn clone(&self) -> Self{
        *self
    }
}

impl<'a, T> Copy for MySlice<'a, T> {}

pub struct MySliceMut<'a , T : 'a> {
    ptr : *const T,
    len : usize,
    data : PhantomData<&'a mut T>
}

impl<'a, T> MySliceMut<'a, T> {
    pub fn from(vec : &'a mut MyVec<T>) -> Self {
        MySliceMut {
            ptr : vec.ptr.as_ptr(),
            len : vec.len,
            data : PhantomData,
        }
    }

    pub fn empty_slice() -> Self {
        MySliceMut {
            ptr : ptr::NonNull::dangling().as_ptr(),
            len : 0,
            data : PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn at(&mut self, index : usize) -> Option<&/*'a putting 'a here would be disastrous*/mut T> {
        if self.len == 0 || index >= self.len {
            None
        }
        else {
            unsafe {
                Some(&mut (*(self.ptr.add(index) as *mut T)))
            }
        }
    }

    pub fn split_at(& mut self, index : usize) -> (MySliceMut<'_, T>, MySliceMut<'_, T>) {
        assert!(index < self.len);
        // 0 : index-1 -> index
        // index : len-1 -> len-index
        let fst = MySliceMut {
            ptr : self.ptr,
            len : index,
            data : PhantomData,
        };
        let snd = unsafe {
            MySliceMut {
                ptr : self.ptr.add(index),
                len : self.len - index,
                data : PhantomData,
            }
        };
        (fst, snd)
    }

    pub fn chunks_mut(&mut self, chunk_size : usize) -> MyChunksMut<'_, T> {
        assert!(chunk_size <= self.len && chunk_size > 0);
        MyChunksMut {
            ptr : self.ptr,
            len : self.len,
            chunk_size : chunk_size,
            data : PhantomData,            
        }
    }
}

pub struct MyChunksMut<'a, T : 'a> {
    ptr : *const T,
    len : usize,
    chunk_size : usize,
    data : PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for MyChunksMut<'a, T> {
    type Item = MySliceMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        }
        else {
            let chunk_size = cmp::min(self.chunk_size, self.len);
            let res = MySliceMut {
                ptr : self.ptr,
                len : chunk_size,
                data : PhantomData,
            };
            unsafe {
                self.ptr = self.ptr.add(chunk_size);
                self.len = self.len - chunk_size;
            }
            Some(res)
        }
    }
}