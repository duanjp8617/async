use crate::my::myvec;

struct Wtf {
    x : u8,
    y : i32,
}

impl Drop for Wtf {
    fn drop(&mut self) {
        println!("dropping {}, {}", self.x, self.y);
    }
}

pub fn test_my_vec() {
    let mut v = myvec::MyVec::<Box<Wtf>>::new();
    v.insert(0, Box::new(Wtf{x : 5, y : 6}));
    v.insert(0, Box::new(Wtf{x : 7, y : 8}));
    v.insert(1, Box::new(Wtf{x : 9, y : 10}));
    v.insert(3, Box::new(Wtf{x : 100, y : 88}));
    
    let mut v = myvec::MyVec::<Box<Wtf>>::new();
    v.insert(0, Box::new(Wtf{x : 5, y : 6}));
    v.insert(0, Box::new(Wtf{x : 7, y : 8}));
    v.insert(1, Box::new(Wtf{x : 9, y : 10}));
    v.insert(3, Box::new(Wtf{x : 100, y : 88}));
    let e = v.remove(0);
    assert!(e.x == 7);
    assert!(e.y == 8);
    let e = v.remove(2);
    assert!(e.x == 100);
    assert!(e.y == 88);
    
    let mut v = myvec::MyVec::<Box<Wtf>>::new();
    v.insert(0, Box::new(Wtf{x : 5, y : 6}));
    v.insert(0, Box::new(Wtf{x : 7, y : 8}));
    v.insert(1, Box::new(Wtf{x : 9, y : 10}));
    v.insert(3, Box::new(Wtf{x : 100, y : 88}));
    let mut iter = v.into_iter();
    iter.next().map(|e| {
        assert!(e.x == 7);
        assert!(e.y == 8);        
    });
    iter.next().map(|e| {
        assert!(e.x == 9);
        assert!(e.y == 10);        
    });
    iter.next_back().map(|e| {
        assert!(e.x == 100);
        assert!(e.y == 88);       
    });
    iter.next_back().map(|e| {
        assert!(e.x == 5);
        assert!(e.y == 6);       
    });
    assert!(iter.next().is_none(), true);
    assert!(iter.next_back().is_none(), true);

    let mut v = myvec::MyVec::<Box<Wtf>>::new();
    v.insert(0, Box::new(Wtf{x : 5, y : 6}));
    v.insert(0, Box::new(Wtf{x : 7, y : 8}));
    v.insert(1, Box::new(Wtf{x : 9, y : 10}));
    v.insert(3, Box::new(Wtf{x : 100, y : 88}));
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
    v.push(Box::new(Wtf{x : 100, y : 88}));
}