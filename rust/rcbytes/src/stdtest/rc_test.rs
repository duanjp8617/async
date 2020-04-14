use std::rc::Rc;

struct Wtf {
    i : i32,
    j : i32,
}

impl Drop for Wtf {
    fn drop(&mut self) {
        println!("Dropping Wtf object with i: {}, j: {}", self.i, self.j);
    }
}

pub fn try_unwrap() {
    let rc1 = Rc::new(Wtf {i: 1, j: 1});
    let res = Rc::try_unwrap(rc1).map(|inner| {
        println!("The inner object is i: {}, j: {}", inner.i, inner.j);
    });
    match res {
        Ok(_) => {
            println!("Should be Ok");
        },
        Err(_) => {
            println!("Should never be err");
        },
    };
    

    let rc2 = Rc::new(Wtf{i: 2, j: 2});
    let rc3 = rc2.clone();
    let res = Rc::try_unwrap(rc2).map_err(|inner| {
        println!("inner is a rc to Wtf object i: {}, j: {}", inner.i, inner.j);
    });
    println!("res should be Err: {}", res.is_err());
    println!("use rc3 to prevent deadcode alert, i: {}", rc3.i);
}

pub fn from_raw_into_raw() {
    let rc1 = Rc::new(Wtf{i: 1, j: 1});
    let rc2 = rc1.clone();
    let rc3 = rc1.clone();

    let ptr = Rc::into_raw(rc1);
    println!("strong count: {}, weak count: {}", Rc::strong_count(&rc2), Rc::weak_count(&rc2));
    unsafe{println!("The raw pointer is i: {}, j: {}", (*ptr).i, (*ptr).j)};

    // if we comment this line out, the Wtf object will not be dropped
    let rc1 = unsafe{Rc::from_raw(ptr)};

    println!("use rc1 and rc3 to prevent deadcode alert, i: {}, j: {}", rc1.i, rc3.j);
}