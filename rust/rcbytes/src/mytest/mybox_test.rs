use crate::my::mybox::MyBox;

#[derive(Clone)]
struct Wtf {
    i : i32,
    j : i32,
}

impl Drop for Wtf {
    fn drop(&mut self) {
        println!("wtf object with i: {}, j: {} is dropped", self.i, self.j);
    }
}


pub fn new_drop() {
    let mut boxed_obj = MyBox::new(Wtf {
        i : 100,
        j : 99,
    });
    println!("Wtf?");
    boxed_obj.i = 999;
    boxed_obj.j = 1024;
    println!("{}, {}", &boxed_obj.i, &boxed_obj.j);
}

pub fn raw() {
    let boxed_obj = MyBox::new(Wtf {
        i : 100,
        j : 99,
    });

    let ptr = boxed_obj.into_raw();
    unsafe {(*ptr).i = 1024};
    unsafe {(*ptr).j = 256};
    let boxed_obj = unsafe {
        MyBox::from_raw(ptr)
    };
    println!("{}, {}", &boxed_obj.i, &boxed_obj.j);
    drop(boxed_obj);
}

pub fn leak() {
    let boxed_obj = MyBox::new(Wtf {
        i : 100,
        j : 99,
    });

    let ref1 = MyBox::leak(boxed_obj);
    ref1.i = 1024;
    ref1.j = 96;
    println!("I will never see this object dropping: {}, {}", &ref1.i, &ref1.j);
}