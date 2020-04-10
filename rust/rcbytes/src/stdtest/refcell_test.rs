use std::cell::{RefCell, UnsafeCell, Cell, Ref, RefMut};
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
    let v1 = UnsafeCell::new(Wtf {
        i : 1,
        j : 1024,
    });

    let mut inner_obj = v1.into_inner();
    inner_obj.i = 5;
    inner_obj.j = 6;
}

pub fn unsafe_cell2() {
    // The code presented here is strictly forbiden in Rust
    // The problem is that we managed to create a mutable reference 
    // ref_mut to the data contained in the UnsafeCell and mutate the data,
    // while two imutable references are still alive. 

    let v1 = UnsafeCell::new(Wtf {
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
}

pub fn cell1() {
    let cell1 = Cell::new(Wtf {
        i : 5,
        j : 1025
    });

    println!("before cell1.set");
    cell1.set(Wtf {
        i : 7,
        j : 1027,
    });
    println!("after cell1.set");

    let cell2 = Cell::new(Wtf {
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
    let mut cell1 = Cell::new(Wtf{
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
    // let cell2 = Cell::new(Wtf {
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
    let cell1 = Cell::new(WtfCopy{
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

pub fn cell4() {
    let slice1 = &mut [1,2,3,4][..];
    let cell1 = Cell::from_mut(slice1);
    let slice2 = cell1.as_slice_of_cells();

    slice2[0].set(5);
    slice2[1].set(6);
    slice2[2].set(7);
    slice2[3].set(8);

    println!("{:?}", slice1);
}

pub fn refcell1() {
    let ref_cell1 = RefCell::new(Wtf{
        i: 1,
        j: 2,
    });

    let borrow1 = ref_cell1.borrow();
    let borrow2 = ref_cell1.borrow();

    println!("{}, {}", &borrow1.i, &borrow2.j);

    // if we don't drop borrow1 and borrow2 here
    // we will not be able to call into_inner, as 
    // they will be dropped before the entire function
    // returns and extend the borrow to ref_cell1 until
    // the end of this function
    drop(borrow1);  
    drop(borrow2);
    
    let sth = ref_cell1.into_inner();
    println!("{}, {}", sth.i, sth.j);
    drop(sth);

    println!("done");
}

pub fn refcell2() {
    let ref_cell1 = RefCell::new(Wtf{
        i: 1,
        j: 2,
    });

    let borrow1 = ref_cell1.borrow();
    let borrow2 = ref_cell1.borrow();

    println!("{}, {}", &borrow1.i, &borrow2.j);

    // if we don't drop borrow1 and borrow2 here
    // the replace method will panic as it will try 
    // to call borrow_mut
    drop(borrow1);  
    drop(borrow2);
    
    let sth = ref_cell1.replace(Wtf {
        i: 3,
        j: 4,
    });
    println!("{}, {}", sth.i, sth.j);
    drop(sth);

    println!("done");
}

pub fn refcell3() {
    let ref_cell1 = RefCell::new(Wtf{
        i: 1,
        j: 2,
    });

    let borrow1 = ref_cell1.borrow();
    let borrow2 = ref_cell1.borrow();

    println!("{}, {}", &borrow1.i, &borrow2.j);

    // if we don't drop borrow1 and borrow2 here
    // the replace_with method will panic as it will try 
    // to call borrow_mut
    drop(borrow1);  
    drop(borrow2);
    
    let sth = ref_cell1.replace_with(|mut_ref| {
        mut_ref.i += 1;
        mut_ref.j += 1;

        Wtf {
            i: mut_ref.i,
            j: mut_ref.j,
        }
    });
    println!("{}, {}", sth.i, sth.j);
    drop(sth);

    println!("done");
}

pub fn refcell4() {
    let ref_cell1 = RefCell::new(Wtf{
        i: 1,
        j: 2,
    });

    let ref_cell2 = RefCell::new(Wtf{
        i: 2,
        j: 3,
    });

    let borrow1 = ref_cell1.borrow();
    let borrow2 = ref_cell2.borrow();

    println!("{}, {}", &borrow1.i, &borrow1.j);
    println!("{}, {}", &borrow2.i, &borrow2.j);


    // if we don't drop either of borrow1 and borrow2 here
    // the swap method will panic as it will try 
    // to call borrow_mut
    drop(borrow1);  
    drop(borrow2);
    
    ref_cell1.swap(&ref_cell2);

    drop(ref_cell1.into_inner());
    drop(ref_cell2.into_inner());
    
    println!("done");
}

pub fn refcell5() {
    let mut refcell1 = RefCell::new(Wtf {
        i: 1,
        j: 2,
    });

    let ref_mut = refcell1.get_mut();
    ref_mut.i = 5;
    ref_mut.j = 5;

    let borrow1 = refcell1.borrow();
    let res = unsafe {refcell1.try_borrow_unguarded().unwrap()};

    println!("{}, {}", &borrow1.i, &borrow1.j);
    println!("{}, {}", &res.i, &res.j);

    drop(borrow1);
    let mut mut_borrow1 = refcell1.borrow_mut();
    mut_borrow1.i = 1024;
    mut_borrow1.j = 1024;

    // You can easily break the promises of RefCell 
    println!("We can still without dropping mut_borrow1, {}, {}", &res.i, &res.j);

    mut_borrow1.i = 1026;
    mut_borrow1.j = 1026;
}

pub fn refcell6() {
    let cell1 = RefCell::new([1,2,3,4,5,6]);
    let borrow1 = cell1.borrow();

    let borrow2 = Ref::map(borrow1, |orig| {
        &orig[..]
    });

    println!("first");
    for i in borrow2.iter() {
        println!("{}", i);
    }

    println!("second");
    let (fst, snd) = Ref::map_split(borrow2, |orig| {
        orig.split_at(2)
    });

    for i in fst.iter() {
        println!("{}", i);
    }

    for i in snd.iter() {
        println!("{}", i);
    }
}

pub fn refcell7() {
    let cell1 = RefCell::new([1,2,3,4,5,6]);
    let borrow1 = cell1.borrow_mut();

    let mut borrow2 = RefMut::map(borrow1, |orig| {
        &mut orig[..]
    });

    println!("first");
    for i in borrow2.iter_mut() {
        *i += 1;
        println!("{}", i);
    }

    println!("second");
    let (mut fst, mut snd) = RefMut::map_split(borrow2, |orig| {
        orig.split_at_mut(2)
    });

    for i in fst.iter_mut() {
        *i += 1;
        println!("{}", i);
    }

    for i in snd.iter_mut() {
        *i += 1;
        println!("{}", i);
    }

    drop(fst);
    drop(snd);

    println!("final");
    for i in cell1.borrow().iter() {
        println!("{}", i);
    }
}

pub fn refcell8() {
    let cell1 = RefCell::new([1,2,3,4,5,6]);

    // This doesn't work, as Rust will complain about temporary variable being dropped
    // let iter = cell1.borrow().iter();

    let sth = cell1.borrow();
    let iter = sth.iter();

    // cell1.borrow_mut()[2] = 1024;

    for i in iter {
        println!("{}", i);
    }
}