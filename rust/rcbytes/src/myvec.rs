use std::ptr::{Unique, self};
use std::mem;
use std::alloc::{Global, AllocRef, Layout};
use std::ops::Deref;
use std::ops::DerefMut;
use std::marker::PhantomData;

pub struct MyVec<T> {
    pub(crate) ptr : Unique<T>,
    pub (crate) cap : usize,
    pub (crate) len : usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We are not ready to handle ZSTs");
        MyVec {
            ptr : Unique::empty(),
            cap : 0,
            len : 0,
        }
    }

    pub fn grow(&mut self) {
        unsafe {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let (new_cap, ptr) = if self.cap == 0 {
                assert!(elem_size < isize::MAX as usize, "capacity overflow");

                let layout = Layout::from_size_align(elem_size, align).unwrap();
                println!("initial layout: {}, {}", layout.size(), layout.align());
                let result = Global::alloc(&mut Global, layout);

                match result {
                    Ok((ptr, size)) => {
                        println!("allocating {} bytes of memory, the cap becomes {}", size, size/elem_size);
                        (size/elem_size, ptr.cast())
                    },
                    Err(_) => {
                        panic!("allocation error")
                    },
                }
            }
            else {
                let old_bytes_cnt = self.cap * elem_size;
                assert!(old_bytes_cnt < isize::MAX as usize / 2, "capacity overflow");
                
                let layout = Layout::from_size_align(old_bytes_cnt, align).unwrap();
                println!("current layout: {}, {}", layout.size(), layout.align());
                let result = Global::realloc(&mut Global, ptr::NonNull::from(self.ptr).cast(), layout, old_bytes_cnt*2);
                match result {
                    Ok((ptr, size)) => {
                        println!("allocating {} bytes of memory, the cap becomes {}", size, size/elem_size);
                        (size/elem_size, ptr.cast())
                    },
                    Err(_) => {
                        panic!("allocation error")
                    },
                }
            };

            self.cap = new_cap;
            self.ptr = ptr.into();
        }
    }
}

impl<T> MyVec<T> {
    pub fn push(&mut self, elem : T) {
        if self.cap == self.len {
            self.grow();
        }

        unsafe {
            let ptr = self.ptr.as_ptr();
            ptr::write(ptr.add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option <T> {
        if self.len == 0 {
            None
        }
        else {
            self.len -= 1;
            unsafe {
                Some (ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {};

            let elem_size = mem::size_of::<T>();
            let align = mem::align_of::<T>();
            let total_bytes = elem_size * self.cap;
            let layout = Layout::from_size_align(total_bytes, align).unwrap();
            unsafe{
                Global::dealloc(&mut Global, ptr::NonNull::from(self.ptr).cast(), layout);
            }
            println!("The memory is deallocated");
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> MyVec<T> {
    pub fn insert(&mut self, pos : usize, elem : T) {
        assert!(pos <= self.len, "invalid insert position");
        
        if self.len == self.cap {
            self.grow()
        }

        unsafe {
            ptr::copy(self.ptr.as_ptr().add(pos), self.ptr.as_ptr().add(pos+1), self.len-pos);
            ptr::write(self.ptr.as_ptr().add(pos), elem);
        }

        self.len+=1;       
    }

    pub fn remove(&mut self, pos : usize) -> T {
        assert!(pos < self.len, "invalid remove position");
        
        let elem = unsafe {ptr::read(self.ptr.as_ptr().add(pos))};

        unsafe {            
            ptr::copy(self.ptr.as_ptr().add(pos+1), self.ptr.as_ptr().add(pos), self.len-pos-1);
        }

        self.len -= 1;
        elem
    }
}

pub struct IntoIter<T> {
    ptr : Unique<T>, 
    cap : usize,
    begin : *const T,
    end : *const T,
}

impl<T> MyVec<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        let ptr = self.ptr;
        let cap = self.cap;
        let len = self.len;

        mem::forget(self);
        
        unsafe {
            IntoIter {
                ptr : ptr,
                cap : cap,
                begin : ptr.as_ptr(),
                end : ptr.as_ptr().add(len),
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin == self.end {
            None
        }
        else {
            unsafe {
                let elem = ptr::read(self.begin);
                self.begin = self.begin.add(1);
                Some(elem)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_elems = (self.end as usize - self.begin as usize) / mem::size_of::<T>();
        (remaining_elems, Some(remaining_elems))
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.begin != self.end {
            while let Some(_) = self.next() {};
        };

        let elem_size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        let total_bytes = elem_size * self.cap;
        let layout = Layout::from_size_align(total_bytes, align).unwrap();
        unsafe{
            Global::dealloc(&mut Global, ptr::NonNull::from(self.ptr).cast(), layout);
        }            
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.begin == self.end {
            None
        }
        else {            
            unsafe {
                self.end = self.end.sub(1);
                Some(ptr::read(self.end))
            }
        }
    }
}

pub struct Drain<'a, T : 'a> {
    phantom_vec : PhantomData<&'a mut MyVec<T>>,
    begin : *const T,
    end : *const T,
}

impl<T> MyVec<T> {
    pub fn drain<'a>(&'a mut self) -> Drain<'a, T> {
        let len = self.len;
        self.len = 0;
        
        unsafe {
            Drain {
                phantom_vec : PhantomData,
                begin : self.ptr.as_ptr(),
                end : self.ptr.as_ptr().add(len),
            }
        }
    }
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin == self.end {
            None
        }
        else {
            unsafe {
                let elem = ptr::read(self.begin);
                self.begin = self.begin.add(1);
                Some(elem)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining_elems = (self.end as usize - self.begin as usize) / mem::size_of::<T>();
        (remaining_elems, Some(remaining_elems))
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        if self.begin != self.end {
            while let Some(_) = self.next() {};
        };            
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.begin == self.end {
            None
        }
        else {            
            unsafe {
                self.end = self.end.sub(1);
                Some(ptr::read(self.end))
            }
        }
    }
}