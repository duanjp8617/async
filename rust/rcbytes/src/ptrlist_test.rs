use crate::ptrlist;

struct Wtf {
    x : u8,
    y : i32,
}

impl Drop for Wtf {
    fn drop(&mut self) {
        println!("dropping {}, {}", self.x, self.y);
    }
}

pub fn test_ptr_myls() {
    let mut ls = ptrlist::MyLs::new();
    ls.push_front(Box::new(Wtf{x : 5, y : 1}));
    ls.push_front(Box::new(Wtf{x : 6, y : 2}));
    ls.peek_front().map(|val|{assert!(val.x == 6)});
    ls.pop_front().map(|val|{assert!(val.x == 6)});
    println!("fk");
    ls.peek_front().map(|val|{assert!(val.x == 5)});
}
