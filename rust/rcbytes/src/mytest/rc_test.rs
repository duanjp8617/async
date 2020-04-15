use crate::my::rc::{Rc, Weak};
use std::cell::RefCell;

struct Wtf {
    i : i32,
    j : i32,
}

impl Drop for Wtf {
    fn drop(&mut self) {
        println!("Dropping Wtf object with i: {}, j: {}", self.i, self.j);
    }
}

pub fn test_rc() {
    let rc1 = Rc::new(Wtf{i: 1, j: 1});
    let rc2 = rc1.clone();
    let rc3 = rc1.clone();

    {
        println!("dropping rc1");
        drop(rc1);
        println!("done");
    }
    {
        println!("dropping rc2");
        drop(rc2);
        println!("done");
    }
    {
        println!("dropping rc3");
        println!("access elements of the Wtf object i: {}, j: {}", rc3.i, rc3.j);
        drop(rc3);
        println!("done");
    }
}

pub fn downgrade_upgrade() {
    let rc1 = Rc::new(Wtf{i: 1, j: 1});
    let rc2 = rc1.clone();
    let weak1 = Rc::downgrade(&rc2);
    {
        drop(rc1);
        drop(rc2);
        // 
    }

    println!("out of scope");
    let res = weak1.upgrade();
    println!("should be none: {}", res.is_none());
}

enum Circle {
    Empty,
    Something(Wtf, Rc<RefCell<Circle>>)
}

pub fn test_circle() {
    let strong1 = Rc::new(RefCell::new(Circle::Empty));
    let strong2 = Rc::new(RefCell::new(Circle::Something(Wtf{i: 1, j: 1}, strong1.clone())));

    // If we comment out the following two lines and break the circle, we 
    // can see the Wtf object being dropped

    let mut ref_mut = strong1.borrow_mut();
    *ref_mut = Circle::Something(Wtf{i: 2, j: 2}, strong2.clone());

    // However, after adding the previous two lines, a reference circle is created
    // between the two RefCells, none of the Wtf objects will ever get dropped
}

enum StrongObj {
    Something(Wtf, Rc<RefCell<WeakObj>>),
}

enum WeakObj {
    Empty,
    Something(Wtf, Weak<RefCell<StrongObj>>),
}

pub fn circle_breaker() {
    let weak_obj1 = Rc::new(RefCell::new(WeakObj::Empty));
    let strong_obj1 = Rc::new(RefCell::new(StrongObj::Something(Wtf{i: 1, j: 1}, weak_obj1.clone())));

    let mut ref_mut = weak_obj1.borrow_mut();
    *ref_mut = WeakObj::Something(Wtf{i: 2, j: 2}, Rc::downgrade(&strong_obj1));
    // We should see both strong object and weak object being dropped 
    // if we comment the rest of the code out
    
    drop(ref_mut);
    drop(strong_obj1);

    let ref1 = weak_obj1.borrow();
    match *ref1 {
        WeakObj::Empty => {

        },
        WeakObj::Something(ref wtf_obj, ref weak_ptr) => {
            println!("weak_obj is i: {}, j: {}", wtf_obj.i, wtf_obj.j);
            let opt = weak_ptr.upgrade();
            println!("weak pointer can not upgrade: {}", opt.is_none());
        },
    };
}