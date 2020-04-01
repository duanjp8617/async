use std::iter::{self, IntoIterator};
use std::convert::TryFrom;

pub fn three_iterators() {
    let mut vec = vec![1,2,3,4,5,6];

    let iter = vec.iter();
    for i in iter {
        println!("{}", i);
    }

    let iter_mut = vec.iter_mut();
    for i in iter_mut {
        *i += 1;
        println!("{}", i);
    }

    let into_iter = vec.into_iter();
    for i in into_iter {
        println!("{}", i);
    }
}

pub fn three_iterators_desugar() {
    let vec = vec![1,2,3,4,5,6];
    
    let iter = vec.iter();
    let mut iter_alias = iter.into_iter();
    while let Some(item) = iter_alias.next() {
        println!("{}", item);
    }

    let mut into_iter = vec.into_iter();
    while let Some(item) = into_iter.next() {
        println!("{}", item);
    }
}

pub fn size_hint() {
    let vec = vec![1,2,3,4];
    let mut iter = vec.iter();
    iter.next();
    iter.next();
    let (lower_bound, upper_bound) = iter.size_hint();
    println!("{}, {}", lower_bound, upper_bound.unwrap());
}

pub fn count() {
    // * implement with fold
    // count is implemented via fold, try it later in fold
    let vec = vec![1,2,3,4];
    let mut iter = vec.iter();
    iter.next();
    iter.next();
    println!("{}", iter.count());
}

pub fn last() {
    // * implement with fold
    // last is implemented via fold as well
    let vec = vec![1,2,3,4,5];
    let iter = vec.iter();
    let last = iter.last();
    last.map(|i|{
        println!("the last is {}", i);
    });
}

pub fn nth() {
    // implement
    let vec = vec![1,2,3,4,5];
    let mut iter = vec.iter();
    iter.nth(0).map(|i|{
        println!("expect 1, {}", i);
    });
    iter.nth(0).map(|i|{
        println!("expect 2, {}", i);
    });
    iter.nth(2).map(|i|{
        println!("expect 5, {}", i);
    });
}

pub fn stepby() {
    // StepBy iterator allows you to skip items in the original iterator
    let vec = vec![1,2,3,4,5];
    let iter  = vec.iter();
    let stepby = iter.step_by(3);
    // let iter_mut = vec.iter_mut();
    for i in stepby {
        println!("{}", i);
    }
}

pub fn chain(){
    // implement
    println!("first");
    let vec = vec![1,2,3,4,5];
    let iter1 = vec.iter().step_by(2);
    let iter2 = iter1.chain(vec.iter().step_by(3));
    for i in iter2 {
        println!("{}", i);
    }

    println!("second");
    let mut vec = vec![1,2,3,4,5];
    let iter1 = vec.iter_mut().step_by(2);
    // let iter2 = iter1.chain(vec.iter_mut().step_by(3));
    let mut vec2 = vec![1,2,3,4,5];
    let iter2 = vec2.iter_mut().step_by(3);
    let chain = iter1.chain(iter2);
    for i in chain {
        *i += 1;
        println!("{}", i);
    }
}

pub fn zip() {
    let vec = vec![1,2,3,4,5];
    let iter1 = vec.iter().step_by(2);
    let iter2 = vec.iter();

    let iter_zip = iter1.zip(iter2);
    for (left, right) in iter_zip {
        println!("left: {}, right: {}", left, right);
    }

    let mut vec1 = vec![1,2,3,4,5];
    let mut vec2 = vec!["1", "2", "3", "4", "5"];
    let iter1 = vec1.iter_mut();
    let iter2 = vec2.iter_mut();
    let iter_zip = iter1.zip(iter2);
    for (fst, snd) in iter_zip {
        *fst += 1;
        snd.chars().nth(0).map(|c|{
            println!("{}", c);
        });
    }
    println!("{:?}", vec1);
}

pub fn map() {
    // implement
    let mut vec = vec![1,2,3,4,5];
    let iter = vec.iter_mut().map(|elem| {
        *elem += 1;
        elem
    });
    for elem in iter {
        *elem += 1;
    }
    for i in vec {
        println!("{}", i);
    }
}

pub fn for_each() {
    // implement with fold
    let mut vec = vec![1,2,3,4,5];
    vec.iter_mut().for_each(|val| {
        *val += 1;
    });
    for i in vec {
        println!("{}", i);
    }
}

pub fn filter() {
    // implement
    let mut vec = vec![1,2,3,4,5];
    vec.iter_mut().filter(|val|{
        **val % 2 == 1
    }).map(|val|{
        *val += 1;
        val
    }).for_each(|val|{
        *val += 1;
        println!("{}", val);
    });
}

pub fn fitler_map() {
    let mut vec = vec![1,2,3,4,5];
    vec.iter_mut().filter_map(|val| {
        if *val % 2 == 1 {
            Some(val)
        }
        else {
            None
        }
    }).map(|val| {
        *val += 1;
        val
    }).for_each(|val|{
        println!("{}", val);
    }); 
    
    let vec = vec!["1", "3", "5", "fuck", "you"];
    let iter = vec.iter().filter_map(|s| {
        s.parse::<i32>().ok()
    });
    iter.for_each(|val| {
        println!("{}", val);
    });

    println!("Another");
    let iter = vec.iter().map(|s|{
        s.parse::<i32>()
    }).filter(|r|{
        r.is_ok()
    }).map(|r|{
        r.unwrap()
    });
    iter.for_each(|val| {
        println!("{}", val);
    });
}
pub fn enumerate() {
    let vec = vec![1,2,3,4,5];
    let iter = vec.iter().enumerate();
    for (fst, snd) in iter {
        println!("index: {}, val: {}", fst, snd);
    }
}

pub fn peekble() {
    let vec = vec![1,2,3,4,5];
    let mut iter = vec.iter().peekable();
    iter.peek().map(|val| {
        println!("{}", *val);
    });
    iter.next().map(|val| {
        println!("{}", val);
    });
}

pub fn skip_while() {
    let mut vec = vec![1,2,3,4,5,6,7];
    let iter = vec.iter_mut().enumerate().skip_while(|(fst, _)| {
        *fst < 4
    }).map(|(_, val)|{
        val
    });

    for i in iter {
        *i += 1;
    }

    println!("{:?}", &vec);
}

pub fn take_while() {
    let mut vec = vec![1,2,3,4,5,6,7];
    let iter = vec.iter_mut().enumerate().take_while(|(fst, _)| {
        *fst < 4
    }).map(|(_, val)|{
        val
    });

    for i in iter {
        *i += 1;
    }

    println!("{:?}", &vec);
}

pub fn map_while() {
    let mut vec = [1, 2, -1, 3];
    let mut iter = vec.iter_mut();
    let iter_map = (&mut iter).map_while(|val| {
        *val += 1;
        u32::try_from(*val).ok()
    });

    for i in iter_map {
        println!("{}", i);
    }
    println!("--------");
    for i in iter {
        println!("{}", i);
    }
}

pub fn skip() {
    let vec = vec![1,2,3,4,5,6,7,8,9];
    let iter = vec.iter().skip(5);
    for i in iter {
        println!("{}", i);
    }
}

pub fn take() {
    let vec = vec![1,2,3,4,5,6,7,8];
    let first_half = vec.iter().take(4);
    let second_half = vec.iter().skip(4);

    for i in first_half {
        println!("{}", i);
    }
    for i in second_half {
        println!("{}", i);
    }
}

pub fn scan() {
    let vec = vec![1,2,3,4,5,6,7];
    let iter = vec.iter().scan(0, |accu, cur| {
        *accu += *cur;
        Some(*accu)
    });

    for i in iter {
        println!("{}", i);
    }
}

pub fn flat_map() {
    // implement
    let v1 = [1,2,3,4,5];
    let v2 = [2,3,4,5,6];
    let v3 = [3,4,5,6,7];

    let slice_vec = vec![&v1[..], &v2[..], &v3[..]];
    let iter = slice_vec.iter().flat_map(|slice|{
        slice.iter()
    });

    for i in iter {
        println!("{}", i);
    }
}

pub fn flatten() {
    // implement
    let mut v1 = [[2,3], [3,4], [5,6]];
    let iter = v1.iter_mut().flatten();
    for i in iter {
        *i += 1;
        println!("{}", i);
    }
    println!("{:?}", &v1);
}

pub fn try_fold() {
    // implement
    let v1 = ["1", "2", "3", "4"];
    let val = v1.iter().try_fold(0, |accu, cur| {
        let cur_i = cur.parse::<i32>().ok()?;
        Some(accu+cur_i)
    });

    val.map(|val| {
        println!("This will never be printed : {}", val);
    });
}

pub fn try_for_each() {
    let v1 = ["1", "2", "3", "4"];
    let res : Result<(), std::num::ParseIntError> = v1.iter().try_for_each(|val|{
        let int = val.parse::<i32>()?;
        println!("Parsing some integer {}", int);
        Ok(())
    });
    res.unwrap();
}

pub fn fold() {
    // implement
    let v1 = ["1", "2", "3", "4", "5", "err", "wtf", "6"];
    let res = v1.iter().filter_map(|s| {
        s.parse::<i32>().ok()
    }).fold(0, |accu, cur| {
        accu + cur
    });
    println!("{}", res);
}