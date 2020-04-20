#[derive(Debug)]
struct Test {
    s : String,
    ptr : *const String,
}

impl Test {
    fn new(s : &str) -> Self {
        Test {
            s : String::from(s),
            ptr : std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.ptr = (&self.s) as *const String;
    }

    fn peek_s(&self) -> &str {
        &self.s
    }

    fn peek_ptr(&self) -> &str {
        unsafe {&(*self.ptr)}
    }
}

#[allow(dead_code)]
pub fn run1() {
    let mut t1 = Test::new("fuck");
    let mut t2 = Test::new("you");

    t1.init();
    t2.init();

    println!("t1 is {} and {}", t1.peek_s(), t1.peek_ptr());
    println!("t2 is {} and {}", t2.peek_s(), t2.peek_ptr());

    std::mem::swap(&mut t1, &mut t2);
    println!("t1 is {} and {}", t1.peek_s(), t1.peek_ptr());
    println!("t2 is {} and {}", t2.peek_s(), t2.peek_ptr());
}