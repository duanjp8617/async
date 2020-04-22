use std::pin::Pin;
use std::marker::PhantomPinned;

struct Test {
    s : String,
    ptr : *const String,
    _marker : PhantomPinned,
}

impl Test {
    fn new(s : &str) -> Pin<Box<Test>> {
        let mut pinned_box = Box::pin(Test {
            s : String::from(s),
            ptr : std::ptr::null(),
            _marker : PhantomPinned,
        });

        let ref_mut = unsafe{pinned_box.as_mut().get_unchecked_mut()};
        ref_mut.ptr = &ref_mut.s as *const String;

        pinned_box
    }

    fn peek_s<'a>(self : Pin<&'a Test>) ->  &'a str {
        &self.get_ref().s
    }

    fn peek_ptr<'a>(self : Pin<&'a Test>) -> &'a str {
        unsafe{& *self.get_ref().ptr}
    }
}

#[allow(dead_code)]
pub fn run1() {
    let t1 = Test::new("fuck");

    println!("t1 is {} and {}", Test::peek_s(t1.as_ref()), Test::peek_ptr(t1.as_ref()));

    let t2 = Test::new("you");

    println!("t2 is {} and {}", Test::peek_s(t2.as_ref()), Test::peek_ptr(t2.as_ref()));

    // compiler complains that unpin is not implemeted, preventing 
    // shit from happening
    // std::mem::swap(t1.get_mut(), t2.get_mut());
}