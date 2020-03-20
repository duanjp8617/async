#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(box_into_raw_non_null)]

mod myvec;
mod myvec_test;

mod linklist;
mod linklist_test;

mod ptrlist;
mod ptrlist_test;

mod slice_test;

mod myslice;
mod myslice_test;

fn run<F>(f : F, func_name : &'static str) where F : Fn()->() {
    println!("begin {}", func_name);
    f();
    println!("----------------------------------------");
}

fn main() {
    run(myvec_test::test_my_vec, "test_my_vec");
    
    run(linklist_test::test_myls, "test_myls");
    
    run(ptrlist_test::test_ptr_myls, "test_ptr_myls");
    
    run(slice_test::slice_size_len, "slice_size_len");
    run(slice_test::slice_split_first, "slice_split_first");
    run(slice_test::empty_slice, "empty_slice");
    run(slice_test::bracket_operator, "bracket_operator");
    run(slice_test::swap, "swap");
    run(slice_test::reverse, "reverse");
    run(slice_test::array_iteration, "array_iteration");
    run(slice_test::slice_iterator_lifetime, "slice_iterator_lifetime");
    run(slice_test::window, "window");
    run(slice_test::chunks, "chunks");
    run(slice_test::chunks_mut, "chunks_mut");
    run(slice_test::split_mut, "split_mut");

    run(myslice_test::test_myslice, "test_myslice");
    run(myslice_test::test_mimic_mut, "test_myslice_mut");
    run(myslice_test::split_at, "split_at");
    run(myslice_test::test_my_chunks_mut, "test_my_chunks_mut");
    
}