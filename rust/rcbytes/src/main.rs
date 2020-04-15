#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(box_into_raw_non_null)]
#![feature(iter_map_while)]
#![feature(try_trait)]
#![feature(never_type)]
#![feature(cell_update)]

mod stdtest;
use crate::stdtest::{box_test, iterator_test, refcell_test, slice_test, rc_test};

mod my;

mod mytest;
use crate::mytest::{linklist_test, mybox_test, myiterator_test, myrefcell_test, myslice_test, myvec_test, ptrlist_test};

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
    run(slice_test::split_at_mut, "split_at_mut");
    run(slice_test::split_mut, "split_mut");
    run(slice_test::sort_and_search, "sort_and_search");
    run(slice_test::rotate_left, "rotate_left");
    run(slice_test::clone_from_slice, "clone_from_slice");
    run(slice_test::copy_from_slice, "copy_from_slice");
    run(slice_test::align_to, "align_to");

    run(myslice_test::test_myslice, "test_myslice");
    run(myslice_test::test_mimic_mut, "test_myslice_mut");
    run(myslice_test::split_at, "split_at");
    run(myslice_test::test_my_chunks_mut, "test_my_chunks_mut");
    run(myslice_test::test_mimic_slice_mut, "test_mimic_slice_mut");
    
    run(box_test::new, "box_test::new");
    run(box_test::interacting_with_raw, "interacting_with_raw");
    run(box_test::leak, "box_test::leak");
    run(box_test::clone, "box_test::clone");

    run(mybox_test::new_drop, "mybox_test::new_drop");
    run(mybox_test::raw, "mybox_test::raw");
    run(mybox_test::leak, "mybox_test::leak");

    run(iterator_test::three_iterators, "three_iterators");
    run(iterator_test::three_iterators_desugar, "three_iterators_desugar");
    run(iterator_test::size_hint, "size_hint");
    run(iterator_test::count, "count");
    run(iterator_test::last, "last");
    run(iterator_test::nth, "nth");
    run(iterator_test::stepby, "stepby");
    run(iterator_test::chain, "chain");
    run(iterator_test::zip, "zip");
    run(iterator_test::map, "map");
    run(iterator_test::for_each, "for_each");
    run(iterator_test::filter, "filter");
    run(iterator_test::fitler_map, "fitler_map");
    run(iterator_test::enumerate, "enumerate");
    run(iterator_test::peekble, "peekble");
    run(iterator_test::skip_while, "skip_while");
    run(iterator_test::take_while, "take_while");
    run(iterator_test::map_while, "map_while");
    run(iterator_test::skip, "skip");
    run(iterator_test::take, "take");
    run(iterator_test::scan, "scan");
    run(iterator_test::flat_map, "flat_map");
    run(iterator_test::flatten, "flatten");
    run(iterator_test::try_fold, "try_fold");
    run(iterator_test::try_for_each, "try_for_each");
    run(iterator_test::fold, "fold");

    run(myiterator_test::iterator_wrapper, "iterator_wrapper");
    run(myiterator_test::nth, "nth");
    run(myiterator_test::chain, "chain");
    run(myiterator_test::map, "map");
    run(myiterator_test::filter, "filter");
    run(myiterator_test::flatten, "flatten");
    run(myiterator_test::flat_map, "flat_map");
    run(myiterator_test::try_fold, "try_fold");
    run(myiterator_test::fold, "fold");
    run(myiterator_test::count, "count");
    run(myiterator_test::last, "last");
    run(myiterator_test::for_each, "for_each");

    run(refcell_test::new, "new");
    run(refcell_test::rc, "rc");
    run(refcell_test::unsafe_cell1, "unsafe_cell1");
    run(refcell_test::unsafe_cell2, "unsafe_cell2");
    run(refcell_test::cell1, "cell1");
    run(refcell_test::cell2, "cell2");
    run(refcell_test::cell3, "cell3");
    run(refcell_test::cell4, "cell4");
    run(refcell_test::refcell1, "refcell1");
    run(refcell_test::refcell2, "refcell2");
    run(refcell_test::refcell3, "refcell3");
    run(refcell_test::refcell4, "refcell4");
    run(refcell_test::refcell5, "refcell5");
    run(refcell_test::refcell6, "refcell6");
    run(refcell_test::refcell7, "refcell7");
    run(refcell_test::refcell8, "refcell8");

    run(myrefcell_test::unsafe_cell1, "unsafe_cell1");
    run(myrefcell_test::unsafe_cell_strictly_forbidden, "unsafe_cell_strictly_forbidden");
    run(myrefcell_test::cell1, "cell1");
    run(myrefcell_test::cell2, "cell2");
    run(myrefcell_test::cell3, "cell3");
    run(myrefcell_test::new, "new");
    run(myrefcell_test::rc, "rc");
    run(myrefcell_test::map, "map");

    run(rc_test::try_unwrap, "try_unwrap");
    run(rc_test::from_raw_into_raw, "from_raw_into_raw");
    run(rc_test::downgrade_upgrade, "downgrade_upgrade");
    run(rc_test::test_circle, "test_circle");
    run(rc_test::circle_breaker, "circle_breaker");
}