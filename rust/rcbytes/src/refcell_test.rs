use std::cell::RefCell;

pub fn new() {
    println!("test");
    let cell = RefCell::new(1024);
    let borrow = cell.borrow();
    println!("{}", borrow);
    drop(borrow);
    let mut borrow_mut = cell.borrow_mut();
    *borrow_mut = 1025;
    println!("{}", borrow_mut);
}