use crate::myslice::{MySlice, MySliceMut};
use crate::myvec::MyVec;

pub fn test_myslice() {
    let mut ls = MyVec::new();
    ls.push(1);
    ls.push(2);
    ls.push(3);

    let slice = MySlice::from(&ls);
    let copy_slice = slice;

    slice.at(0).map(|val|{
        println!("{}", val);
    });

    // drop(ls);
    // ls.push(1);

    copy_slice.at(0).map(|val|{
        println!("{}", val);
    });

    let ref1 = slice.at(0).unwrap();
    let ref2 = slice.at(0).unwrap();
    println!("{}, {}", ref1, ref2);
}

pub fn test_mimic_mut() {
    println!("begin");
    let mut ls = MyVec::new();
    ls.push(1);
    ls.push(2);
    ls.push(3);
    assert!(ls[0] == 1);
    assert!(ls[1] == 2);
    assert!(ls[2] == 3);

    let mut slice_mut = MySliceMut::from(&mut ls);
    let mut_ref1 = slice_mut.at(0).unwrap();
    *mut_ref1 = *mut_ref1 + 1;
    assert!(ls[0] == 2);

    // error case 1:
    // If we borrow from a temporary variable and then use the borrow
    // rust will complain
    // let mut_ref1 = MySliceMut::from(&mut ls).at(0).unwrap();
    // *mut_ref1 = *mut_ref1 + 1;
    
    // error case 2:
    // if at method returns Option<&'a mut T>, then we can create two
    // mutable references to the same element stored in the vec, violating 
    // Rust's aliasing restriction.
    // let mut slice_mut = MySliceMut::from(&mut ls);
    // let mut_ref1 = slice_mut.at(0).unwrap();
    // let mut_ref2 = slice_mut.at(0).unwrap();
    // *mut_ref1 = *mut_ref2 + 1;
}

pub fn split_at() {
    let mut vec = MyVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    assert!(vec[0] == 1);
    assert!(vec[1] == 2);
    assert!(vec[2] == 3);
    assert!(vec[3] == 4);

    let mut slice = MySliceMut::from(&mut vec);
    let (mut fst, mut snd) = slice.split_at(2);
    fst.at(0).map(|val| {        
        assert!(*val == 1);
        println!("{}", val);
    });
    // let (mut fst1, mut snd1)= slice.split_at(2);
    snd.at(0).map(|val| {
        assert!(*val == 3);
        println!("{}", val);
    });

    let (mut fst_fst, mut fst_snd) = fst.split_at(1);
    let (mut snd_fst, mut snd_snd) = snd.split_at(1);
    fst_fst.at(0).map(|val|{
        assert!(*val == 1);
        println!("{}", val);
    });
    fst_snd.at(0).map(|val|{
        assert!(*val == 2);
        println!("{}", val);
    });
    snd_fst.at(0).map(|val|{
        assert!(*val == 3);
        println!("{}", val);
    });
    snd_snd.at(0).map(|val|{
        assert!(*val == 4);
        println!("{}", val);
    });

    // let (_, mut snd) = MySliceMut::from(&mut vec).split_at(2);    
    // snd.at(0).map(|val| {
    //     assert!(*val == 3);
    //     println!("{}", val);
    // });
}

pub fn test_my_chunks_mut() {
    let mut vec = MyVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    assert!(vec[0] == 1);
    assert!(vec[1] == 2);
    assert!(vec[2] == 3);
    assert!(vec[3] == 4);
    assert!(vec[4] == 5);

    let mut slice = MySliceMut::from(&mut vec);
    let mut chunks = slice.chunks_mut(3);
    let mut slice1 = chunks.next().unwrap();
    let mut slice2 = chunks.next().unwrap();

    slice1.at(0).map(|val|{
        assert!(*val == 1);
        println!("{}", val);
    });
    slice1.at(1).map(|val|{
        assert!(*val == 2);
        println!("{}", val);
    });
    slice1.at(2).map(|val|{
        assert!(*val == 3);
        println!("{}", val);
    });
    // let mut chunks1 = slice.chunks_mut(3);
    // drop(vec);

    slice2.at(0).map(|val|{
        assert!(*val == 4);
        println!("{}", val);
    });
    slice2.at(1).map(|val|{
        assert!(*val == 5);
        println!("{}", val);
    });
}