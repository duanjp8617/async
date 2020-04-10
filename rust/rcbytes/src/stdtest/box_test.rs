use std::alloc::{alloc, Layout};

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

pub fn new() {
    let boxed_obj = Box::new(Wtf {
        i : 100,
        j : 101,
    });
    drop(boxed_obj);

    // I will ignore pin and uninit and explore them later
    // let pined_obj = Box::pin(3);
    // let uninit_obj = Box::<i32>::new_uninit();
}

pub fn interacting_with_raw() {
    let layout = Layout::new::<Wtf>();
    let ptr = unsafe {alloc(layout) as *mut Wtf};
    let mut boxed_obj = unsafe {Box::from_raw(ptr)};
    boxed_obj.i = 105;
    boxed_obj.j = 106;
    drop(boxed_obj);

    let boxed_obj = Box::new(Wtf{i : 100, j : 101});
    let _ptr = Box::into_raw(boxed_obj);
    println!("The boxed_obj will be silently leaked without the following code");
    let boxed_obj = unsafe {Box::from_raw(_ptr)};
    drop(boxed_obj);
}

pub fn leak() {
    let boxed_obj = Box::new(Wtf{i : 100, j : 99});
    let ref_to_boxed_obj = Box::leak(boxed_obj);
    ref_to_boxed_obj.i = 5;
    ref_to_boxed_obj.j = 6;
    println!("Content of the boxed_obj is modified to i:{}, j:{}", ref_to_boxed_obj.i, ref_to_boxed_obj.j);
}

pub fn clone() {
    let boxed_obj = Box::new(Wtf{i : 100, j : 99});
    let cloned_obj = boxed_obj.clone();
    drop(boxed_obj);
    drop(cloned_obj);
}


