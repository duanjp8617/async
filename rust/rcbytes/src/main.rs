#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(box_into_raw_non_null)]

mod myvec;
mod linklist;
mod ptrlist;

struct wtf {
    x : u8,
    y : i32,
}

impl Drop for wtf {
    fn drop(&mut self) {
        println!("dropping {}, {}", self.x, self.y);
    }
}

fn print_all(wtfs : &[Box<wtf>]) {
    for w in wtfs {
        println!("{}, {}", w.x, w.y);
    }
}

fn test_my_vec() {
    let mut v = myvec::MyVec::<Box<wtf>>::new();

    // v.push(Box::new(wtf{x : 5, y : 6}));
    // v.push(Box::new(wtf{x : 7, y : 9}));
    // v.push(Box::new(wtf{x : 100, y : 99 }));
    // print_all(&v);
    // println!("{}, {}", v[1].x, v[1].y);
    // v.pop().map(|wtf_val| {
    //     assert!(wtf_val.x == 100);
    //     assert!(wtf_val.y == 99);
    // });
    // println!("Hello, world!"); 

    v.insert(0, Box::new(wtf{x : 5, y : 6}));
    v.insert(0, Box::new(wtf{x : 7, y : 8}));
    v.insert(1, Box::new(wtf{x : 9, y : 10}));
    v.insert(3, Box::new(wtf{x : 100, y : 88}));
    
    // let e = v.remove(0);
    // assert!(e.x == 7);
    // assert!(e.y == 8);

    // let e = v.remove(2);
    // assert!(e.x == 100);
    // assert!(e.y == 88);
    
    // let mut iter = v.into_iter();
    // iter.next().map(|e| {
    //     assert!(e.x == 7);
    //     assert!(e.y == 8);        
    // });
    // iter.next().map(|e| {
    //     assert!(e.x == 9);
    //     assert!(e.y == 10);        
    // });
    // iter.next_back().map(|e| {
    //     assert!(e.x == 100);
    //     assert!(e.y == 88);       
    // });
    // iter.next_back().map(|e| {
    //     assert!(e.x == 5);
    //     assert!(e.y == 6);       
    // });

    // assert!(iter.next().is_none(), true);
    // assert!(iter.next_back().is_none(), true);

    let mut drain = v.drain();
    drain.next().map(|e| {
        assert!(e.x == 7);
        assert!(e.y == 8);        
    });
    drain.next().map(|e| {
        assert!(e.x == 9);
        assert!(e.y == 10);        
    });
    drain.next_back().map(|e| {
        assert!(e.x == 100);
        assert!(e.y == 88);       
    });
    drain.next_back().map(|e| {
        assert!(e.x == 5);
        assert!(e.y == 6);       
    });

    assert!(drain.next().is_none(), true);
    assert!(drain.next_back().is_none(), true);

    drop(drain);

    v.push(Box::new(wtf{x : 100, y : 88}));
}

fn test_myls() {
    let mut ls = linklist::MyLs::new();
    ls.push_front(Box::new(wtf{x : 5, y : 1}));
    ls.push_front(Box::new(wtf{x : 6, y : 2}));
    ls.peek_front().map(|val|{assert!(val.x == 6)});
    ls.pop_front().map(|val|{assert!(val.x == 6)});
    println!("fk");
    ls.peek_front().map(|val|{assert!(val.x == 5)});
}

fn test_ptr_myls() {
    let mut ls = ptrlist::MyLs::new();
    ls.push_front(Box::new(wtf{x : 5, y : 1}));
    ls.push_front(Box::new(wtf{x : 6, y : 2}));
    ls.peek_front().map(|val|{assert!(val.x == 6)});
    ls.pop_front().map(|val|{assert!(val.x == 6)});
    println!("fk");
    ls.peek_front().map(|val|{assert!(val.x == 5)});
}

fn main() {
    println!("wtf?");
    test_ptr_myls();

}