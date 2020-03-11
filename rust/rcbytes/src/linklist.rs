use std::ptr::NonNull;
use std::marker::PhantomData;

struct Node<T> {
    elem : T,
    next : Option<NonNull<Node<T>>>,
}

pub struct MyLs<T> {
    head : Option<NonNull<Node<T>>>,
    data : PhantomData<Box<Node<T>>>,
    len : usize,
}

impl<T> MyLs<T> {
    pub fn new() -> Self {
        MyLs {
            head : None,
            data : PhantomData,
            len : 0,
        }
    }

    pub fn push_front(&mut self, elem : T) {
        let boxed_node = Box::new(Node{
            elem : elem,
            next : self.head.take(),
        });
        self.head = Some(Box::into_raw_non_null(boxed_node));
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        }
        else {
            self.len -= 1;
            let node = self.head.take();
            node.map(|node_ptr|{            
                let boxed_node = unsafe {Box::from_raw(node_ptr.as_ptr())};
                self.head = boxed_node.next;
                boxed_node.elem
            })
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        }
        else {
            self.head.as_ref().map(|node_ptr| {
                unsafe{&node_ptr.as_ref().elem}
            })
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

