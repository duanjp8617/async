use crate::myrefcell::{MyUnsafeCell, MyCell, MyRef, MyRefMut, MyRefCell};
use std::rc::Rc;

struct Wtf {
    i : i32,
    j : i32,
}

impl Drop for Wtf {
    fn drop(&mut self) {
        println!("Wtf object with i: {}, j: {} is dropped", self.i, self.j);
    }
}

pub fn unsafe_cell1(){
    let v1 = MyUnsafeCell::new(Wtf {
        i : 1,
        j : 1024,
    });

    let mut inner_obj = v1.into_inner();
    inner_obj.i = 5;
    inner_obj.j = 6;
    drop(inner_obj);
}

pub fn unsafe_cell_strictly_forbidden() {
    // The code presented here is strictly forbiden in Rust
    // The problem is that we managed to create a mutable reference 
    // ref_mut to the data contained in the UnsafeCell and mutate the data,
    // while two imutable references are still alive. 

    let v1 = MyUnsafeCell::new(Wtf {
        i : 1,
        j : 1024,
    });

    let ref1 = unsafe {& *v1.get()};
    let ref2 = unsafe {& *v1.get()};

    let ref_mut = unsafe {&mut *v1.get()};
    ref_mut.i += 1;
    ref_mut.j += 1;

    println!("{}", ref1.i);
    println!("{}", ref2.j);

    drop(v1);
}

pub fn cell1() {
    let cell1 = MyCell::new(Wtf {
        i : 5,
        j : 1025
    });

    println!("before cell1.set");
    cell1.set(Wtf {
        i : 7,
        j : 1027,
    });
    println!("after cell1.set");

    let cell2 = MyCell::new(Wtf {
        i : 6,
        j : 1026,
    });
    cell2.swap(&cell1);

    // We can't use get to acquire a copy of the data hold by cell2
    // because Wtf does not implement Copy trait
    // let sth = cell2.get();
    // println!("{}", sth.i);

    let sth = cell2.replace(Wtf {i: 0, j: 0});
    println!("expect 7, 1027, {}, {}", sth.i, sth.j);

    let inner = cell2.into_inner();
    println!("expect 0, 0, {}, {}", inner.i, inner.j);

    drop(sth);
    drop(inner);
    drop(cell1);
    println!("done");
}

pub fn cell2() {
    let mut cell1 = MyCell::new(Wtf{
        i: 0,
        j: 0,
    });

    unsafe {
        (*cell1.as_ptr()).i = 1;
        (*cell1.as_ptr()).j = 1;
    }

    let mut_ref = cell1.get_mut();

    println!("expecting 1, 1 : {}, {}", mut_ref.i, mut_ref.j);
    
    // the old do-not-work trick
    // let cell2 = MyCell::new(Wtf {
    //     i: 1,
    //     j: 1,
    // });
    // cell1.swap(&cell2);

    mut_ref.i = 2;
    mut_ref.j = 2;

    let sth = cell1.into_inner();
    println!("expecting 2, 2, {}, {}", sth.i, sth.j);
}

#[derive(Clone, Copy)]
struct WtfCopy {
    i : i32,
    j : i32,
}

pub fn cell3() {
    let cell1 = MyCell::new(WtfCopy{
        i: 0,
        j: 0,
    });

    let obj = cell1.get();
    println!("{}, {}", obj.i, obj.j);

    let updated_obj = cell1.update(|mut old| {
        old.i += 1;
        old.j += 1;
        old
    });

    println!("{}, {}", updated_obj.i, updated_obj.j);
    println!("{}, {}", cell1.get().i, cell1.get().j);
}

pub fn new() {
    println!("test");
    let cell = MyRefCell::new(1024);
    let borrow = cell.borrow();
    println!("{}", *borrow);
    drop(borrow);
    let mut borrow_mut = cell.borrow_mut();
    *borrow_mut = 1025;
    println!("{}", *borrow_mut);
}

fn mutate_rc_refcell(item : Rc<MyRefCell<i32>>) {
    let mut mut_ref = item.borrow_mut();
    *mut_ref += 1;
}

pub fn rc() {
    let item = Rc::new(MyRefCell::new(1024));
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

    println!("should be 1027, {}", *item.borrow());
}

pub fn map() {
    let item = MyRefCell::new(Wtf{i: 1, j: 2});

    {
        let borrow1 = item.borrow();
        let borrow2 = MyRef::map(borrow1, |val| {
            &val.i
        });

        println!("should be 1 {}", *borrow2);
    }

    {
        let borrow1 = item.borrow_mut();
        let borrow2 = MyRefMut::map(borrow1, |mut_val| {
            mut_val.i += 1;
            &mut mut_val.i
        });

        println!("should be 2, {}", *borrow2);
    }

    drop(item);
    println!("done");
}