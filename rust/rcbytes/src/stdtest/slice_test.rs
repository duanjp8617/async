use std::mem;

fn print_slice(slice : &[i32]) {
    for i in slice{
        println!("{}", i);
    }
}

fn move_in_array(mut arr : [i32; 8]) {
    for i in &mut arr {
        *i = *i + 1;
        println!("{} ", i);
    }
}

pub fn slice_size_len() {
    println!("{}", mem::size_of::<&[i32]>());
    // println!("{}", mem::size_of::<[i32]>());
    let sth = [1,2,3];
    let slice = &sth[0..3];

    println!("slice.len");
    println!("{}",slice.len());
    assert!(slice.first() == Some(&1));
}

pub fn slice_split_first() {
    let slice = &mut [1,2,3,4,5][..];
    slice.split_first().map(|(fst_elem, rest_slice)| {
        println!("{}", fst_elem);
        println!("rest");
        print_slice(rest_slice);
    });
}

pub fn empty_slice() {
    let empty_slice : &[i32] = &[];
    println!("{}", empty_slice.len());
}

pub fn bracket_operator() {
    let array_boxed = [Box::new(1), Box::new(2), Box::new(3)];
    let slice = &array_boxed[0..3];
    let v1 = &slice[2];
    // why deref doesn't work here
    assert!(&**v1 == &3);
}

pub fn swap() {
    println!("swap");
    let mut sth = [1,2,3,4,5];
    let mut_slice = &mut sth[0..3];
    mut_slice.swap(0, 2);
    print_slice(&mut_slice);
}

pub fn reverse() {
    println!("reverse");
    let mut array = [1,2,3,4,5,6,7,8];
    move_in_array(array);
    drop(array);
    let slice = &mut array[0..];
    slice.reverse();
    print_slice(&slice);
}

pub fn array_iteration() {
    println!("1");
    let array = [2,3,4,5,6,7];
    let mut iter = array.iter();
    while let Some(wtf) = iter.next(){
        println!("{}", wtf);
    }

    println!("2");
    let mut array = [2,3,4,5,6,7];
    let mut iter = array.iter_mut();
    while let Some(wtf) = iter.next() {
        *wtf = *wtf + 1;
        println!("{}", wtf);
    }

    println!("3");
    let array = [2,3,4,5,6,7];
    for i in &array {
        println!("{}", i);
    }

    println!("4");
    let mut array = [2,3,4,5,6,7];
    for i in &mut array {
        *i = *i + 1;
        println!("{}", i);
    }

    println!("5");
    let array = [2,3,4,5,6,7];
    let slice = & array[..];
    for i in slice {
        println!("{}", i);
    }

    println!("6");
    let mut array = [2,3,4,5,6,7];
    let slice = &mut array[..];
    for i in slice {
        *i = *i + 1;
        println!("{}", i);
    }

    println!("7");
    let array = [2,3,4,5,6,7];
    let slice = &array[..];
    for i in slice.iter() {
        println!("{}", i);
    }

    println!("8");
    let mut array = [2,3,4,5,6,7];
    let slice = &mut array[..];
    for i in slice.iter_mut() {
        *i = *i + 1;
        println!("{}", i);
    }

    println!("9");
    let array = [2,3,4,5,6,7];
    let slice = & array[..];
    let mut iter = slice.iter();
    while let Some(i) = iter.next() {
        println!("{}", i);
    }

    println!("10");
    let mut array = [2,3,4,5,6,7];
    let slice = &mut array[..];
    let mut iter_mut = slice.iter_mut();
    while let Some(i) = iter_mut.next() {
        *i = *i + 1;
        println!("{}", i);
    }
}

pub fn slice_iterator_lifetime() {
    let mut array = [2,3,4,5,6,7];
    let slice = &mut array[..];
    let mut iter_mut = slice.iter_mut();
    
    // i1 does not borrow from iter_mut, it is treated as a
    // a borrow of slice, and extends the lifetime of slice
    let i1 = iter_mut.next().unwrap();
    
    // i2 is similar as i1
    let i2 = iter_mut.next().unwrap();
    
    *i1 = *i1 + 1;
    
    // We can not borrow slice because i1 and i2 are still in use
    // let mut another_iter_mut = slice.iter_mut();
    
    *i2 = *i2 + 1;

    // due to the previous reasons, we are free to modify i1 and i2
    // without errors. Here, the Rust borrow checker allows i1 and i2
    // to simultaneously exists. Hence, it is important for the API designer
    // to ensure that i1 and i2 do not refer to the same content in the original
    // slice

    println!("{}", i1);
    println!("{}", i2);

    // if i borrow from the generated iter_mut temporary variable,
    // the following code would not compile
    let i = slice.iter_mut().next().unwrap();
    *i = *i + 1;
}

pub fn window() {
    // Why we can't implement windows_mut?
    // if we implement windows_mut, we can get two 
    // mutable slices that share a portion of the elements
    // in the original slice.

    let slice = &[1,2,3,4,5][..];
    let wins = slice.windows(3);
    for win in wins {
        println!("start of a window");
        for i in win {
            println!("{}", i);
        }
        println!("{}", &win[0]);
    }
    // assert!(wins.next() == None);

    let slice = &[1,2,3,4,5][..];
    let mut wins = slice.windows(3);
    for win in &mut wins {
        for i in win {
            println!("{}", i);
        }
        println!("{}", &win[0]);
    }

    assert!(wins.next() == None);
}

pub fn chunks() {
    let slice = &[1,2,3,4,5][..];
    let chunks = slice.chunks(3);
    for chunk in chunks {
        println!("a new chunk");
        for i in chunk.iter() {
            println!("{}", i);
        }
    }
}

pub fn chunks_mut() {
    let slice = &mut [1,2,3,4,5][..];
    let mut chunks_mut = slice.chunks_mut(3);
    
    // chunk1 and chunk2 do not borrow from chunks_mut.
    // instead, both of them can be viewed as special (I don't know how to characterize them)
    // borrows from slice and extend the lifetime of the slice
    
    // Let me try to explain why the following code works:
    // Rust does not prevent you from creating mutable borrows, Rust only examines
    // whether the things being borrowed has been borrowed before.
    // I'm suspecting that Rust only inspect whether object on the right hand side
    // of an assignment has been borrowed before, and it will not bother doing additional
    // checking.
    // In the following two lines of code, Rust only tries to inspect whether chunks_mut has 
    // been borrowed before. Since the lifetime of both chunk1 and chunk2 are not tied to 
    // chunks_mut, chunks_mut is never considered borrowed.
    // chunk1 and chunk2 are actually borrowed from slice, since chunk1 is used by the third line
    // how is that possible to borrow chunk2 from slice? Because Rust doesn't check it. When creating 
    // chunk2 variable, Rust only examines whether chunk_mut has been borrowed, it will not 
    // inspect whether slice has been borrowed.
    // Finally, successfully created mutably reference can be freely used anywhere.  
    
    let chunk1 = chunks_mut.next().unwrap();
    let chunk2 = chunks_mut.next().unwrap();
    chunk1[0] = chunk1[0] + 1;
    
    // if we try to borrow from slice, it won't pass compilation
    // let sth = slice.windows(3);

    chunk2[0] = chunk2[0] + 1;
    for i in chunk1.iter() {
        println!("{}", i);
    }
    for i in chunk2.iter() {
        println!("{}", i);
    }

    let chunk = slice.chunks_mut(3).next().unwrap();
    // drop(slice);
    // let chunk1 = slice.chunks_mut(3).next().unwrap();
    for i in chunk {
        *i += 1;
        println!("{}", i);
    }

    // I can not do something like this, [1,2,3,4,5] will be dropped
    // let slice_mut = (&mut [1,2,3,4,5][..]).chunks_mut(3).next().unwrap();
    // for i in slice_mut {
    //     *i = *i + 1;
    //     println!("{}", i)
    // }

    // I can do something like this
    let slice_mut = &mut [1,2,3,4,5][..];
    
    // Note that chunk1 borrow from slice_mut, not from chunks
    let chunk1 = slice_mut.chunks_mut(3).next().unwrap();

    // Note that an attemp to create another chunk will fail, because
    // the bck knows that slice_mut has been borrowed before.
    // reference can be transitive (I don't know if this interpretation is correct)
    // let chunk2 = slice_mut.chunks_mut(3).next().unwrap();    
    for i in chunk1 {
        *i += 1;
        println!("{}", i);
    }
}

pub fn split_at_mut() {
    let slice = &mut [1,2,3,4,5][..];
    let (fst, snd) = slice.split_at_mut(2);
    
    // We can't do neither of these. Because slice has been 
    // borrowed and has an extedned lifetime to the end of this 
    // function.
    // let (haha, hehe) = slice.split_at_mut(2);
    // slice[0] = slice[0] + 1;
    
    fst[0] = fst[0] + 1;
    snd[0] = snd[0] + 1;
    fst[0] = fst[0] + 1;
    snd[0] = snd[0] + 1;
    print_slice(slice);

    slice[0] = slice[0] + 1;
}

pub fn split_mut() {
    let slice = &mut [1,2,3,4,5][..];
    let split = slice.split_mut(|i| {
        *i == 3
    });
    for chunk in split {
        println!("a new chunk");
        for i in chunk {
            *i += 1;
            println!("{}", i);
        }
    }
}

pub fn sort_and_search() {
    let mut vec = vec!(2,1,2,4,3,2,3,2,1,3,2,3,4,5);
    println!("{:?}", &vec);
    let slice = &mut vec[..];
    slice.sort_unstable();
    
    let res = slice.binary_search(&3);
    match res {
        Ok(i) => {
            println!("found {} from index {}", slice[i], i);
        },
        Err(i) => {
            println!("please insert 3 at index {}", i);
        }
    }

    let res = slice.binary_search(&109);
    match res {
        Ok(i) => {
            println!("found {} from index {}", slice[i], i);
        },
        Err(i) => {
            println!("please insert 109 at index {}", i);
            vec.insert(i, 109);
        }
    }

    let slice = &mut vec[..];
    println!("printing the vec after insertion");
    for i in slice {
        println!("{}", i);
    }
}

pub fn rotate_left() {
    let slice = &mut [1,2,3,4,5][..];
    slice.rotate_left(1);
    println!("{:?}", slice);
}

fn manual_clone_from_slice<T : Clone>(dst : &mut [T], src : &[T]) {
    assert!(dst.len() == src.len());
    let len = dst.len();
    for i in 0..len {
        dst[i].clone_from(&src[i]);
    }
}

pub fn clone_from_slice() {
    let slice = &mut [1,2,3,4,5][..];
    let another = &[2,3,4,5,6][..];
    // slice.clone_from_slice(another);
    manual_clone_from_slice(slice, another);
    println!("{:?}", slice);
}

fn manual_copy_from_slice<T : Copy>(dst : &mut [T], src : &[T]) {
    assert!(dst.len() == src.len());
    let len = dst.len();
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), len);
    }
}

pub fn copy_from_slice() {
    let slice = &mut [1,2,3,4,5][..];
    let another = &[2,3,4,5,6][..];
    // slice.copy_from_slice(another);
    manual_copy_from_slice(slice, another);
    println!("{:?}", slice);
}

pub fn align_to() {
    let vec : [u8 ; 7] = [1,2,3,4,5,6,7];

    let (one, two, three) = unsafe {(&vec[..]).align_to::<u16>()};
    
    println!("one");
    for i in one {
        println!("{}", i);
    }

    println!("two");
    for i in two {
        println!("{}", i);
    }
    
    println!("three");
    for i in three {
        println!("{}", i);
    }
}