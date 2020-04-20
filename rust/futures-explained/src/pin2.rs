use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    s : String,
    ptr : *const String,
    _marker : PhantomPinned,
}

impl Test {
    fn new(s : &str) -> Self {
        Test {
            s : String::from(s),
            ptr : std::ptr::null(),
            _marker : PhantomPinned,
        }
    }

    fn init(self : Pin<&mut Self>) {
        let mut_ref = unsafe{self.get_unchecked_mut()};
        mut_ref.ptr = (&mut_ref.s) as *const String;
    }

    fn peek_s<'a>(self : Pin<&'a Self>) -> &'a str {
        & self.get_ref().s
    }

    fn peek_ptr<'a>(self : Pin<&'a Self>) -> &'a str {
        unsafe {& *self.get_ref().ptr}
    }
}

#[allow(dead_code)]
pub fn run1() {
    let mut t1 = Test::new("fuck");
    
    // shadow t1, making the pinned object no longer accessible
    let mut t1 = unsafe{Pin::new_unchecked(&mut t1)};
    Test::init(t1.as_mut());

    println!("t1 is {} and {}", Test::peek_s(t1.as_ref()), Test::peek_ptr(t1.as_ref()));

    let mut t2 = Test::new("fuck");
    
    // shadow t1, making the pinned object no longer accessible
    let mut t2 = unsafe{Pin::new_unchecked(&mut t2)};
    Test::init(t2.as_mut());

    println!("t2 is {} and {}", Test::peek_s(t2.as_ref()), Test::peek_ptr(t2.as_ref()));

    // compiler complains that unpin is not implemeted, preventing 
    // shit from happening
    // std::mem::swap(t1.get_mut(), t2.get_mut());
}