use std::marker::PhantomData;
use std::alloc::{Global, Layout, AllocRef};
use std::mem;
use std::ptr;

struct Node<T> {
    elem : T,
    next : *const Node<T>,
}

pub struct MyLs<T> {
    head : *const Node<T>,
    len : usize,
    data : PhantomData<Node<T>>
}

impl<T> MyLs<T> {
    pub fn new() -> Self {
        MyLs {
            head : 0 as *const Node<T>,
            len : 0,
            data : PhantomData,
        }
    }

    pub fn push_front(&mut self, elem : T) {
        unsafe {
            let layout = Layout::from_size_align(mem::size_of::<Node<T>>(), mem::align_of::<Node<T>>()).unwrap();
            let result = Global::alloc(&mut Global, layout);

            match result {
                Ok((ptr, _)) => {
                    let node_ptr = ptr.cast::<Node<T>>().as_ptr();
                    ptr::write(&mut ((*node_ptr).elem) as *mut T , elem);
                    (*node_ptr).next = self.head;
                    self.head = node_ptr as *const Node<T>;
                    self.len += 1;
                }
                Err(_) => {
                    panic!("allocation error");
                }
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        }
        else {
            unsafe {
                self.len -= 1;
                let head = self.head;
                self.head = (*head).next;
                
                let elem = ptr::read(&((*head).elem) as *const T);

                let layout = Layout::from_size_align(mem::size_of::<Node<T>>(), mem::align_of::<Node<T>>()).unwrap();
                Global::dealloc(&mut Global, ptr::NonNull::from(&(*head)).cast(), layout);

                Some(elem)
            }
        }
    }

    pub fn peek_front(&mut self) -> Option<&T> {
        if self.len == 0 {
            None
        }
        else {
            unsafe {
                Some(&((*self.head).elem))
            }
        }
    }
}

impl<T> Drop for MyLs<T> {
    fn drop(&mut self) {
        if self.len != 0 {
            while let Some(_) = self.pop_front() {};
        }
    }
}