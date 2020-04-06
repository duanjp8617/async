use std::cell::RefCell;
use std::rc::Rc;

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

fn mutate_rc_refcell(item : Rc<RefCell<i32>>) {
    let mut mut_ref = item.borrow_mut();
    *mut_ref += 1;
}

pub fn rc() {
    let item = Rc::new(RefCell::new(1024));
    let item1 = item.clone();
    let item3 = item.clone();

    // if we try to do a mutable borrow here
    // then the program will panic
    // let mut mut_ref = item.borrow_mut();

    mutate_rc_refcell(item1);
    mutate_rc_refcell(item3);

    {
        let mut mut_ref = item.borrow_mut();
        *mut_ref += 1;
    }

    println!("should be 1027, {}", item.borrow());
}