use crate::my::myiterator::MyIterator;

use std::convert::From;

struct IteratorWrapper<I : IntoIterator> {
    inner : I::IntoIter,
}

impl<I : IntoIterator> From<I> for IteratorWrapper<I>  {
    fn from(other : I) -> Self {
        IteratorWrapper {
            inner : IntoIterator::into_iter(other)
        }    
    }
}

impl<I : IntoIterator> MyIterator for IteratorWrapper<I> {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub fn iterator_wrapper() {
    let vec = vec![1,2,3,4,5];
    let mut wrapper : IteratorWrapper<_> = From::from(vec.iter());
    while let Some(i) = wrapper.next() {
        println!("{}", i);
    }
}

pub fn nth() {
    // implement
    let vec = vec![1,2,3,4,5];
    let mut iter : IteratorWrapper<_> = From::from(vec.iter());
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

pub fn chain() {
    let vec = vec![1,2,3,4,5];
    let iter1 : IteratorWrapper<_> = From::from(vec.iter());
    let iter2 : IteratorWrapper<_> = From::from(vec.iter());
    let mut chained_iter = iter1.chain(iter2);
    chained_iter.nth(5).map(|val| {
        println!("expecting 1, {}", val);
    });
    while let Some(val) = chained_iter.next() {
        println!("{}", val);
    }
}

pub fn map() {
    let vec = vec![1,2,3,4,5];
    let iter1 : IteratorWrapper<_> = From::from(vec.iter());
    let iter2 : IteratorWrapper<_> = From::from(vec.iter());
    let mut mapped_iter = iter1.chain(iter2).map(|val|{
        Box::new(val)
    });
    while let Some(boxed_obj) = mapped_iter.next() {
        println!("{}", *boxed_obj);
    }

    println!("again: ----");

    let mut vec1 = vec![1,2,3,4,5,6];
    let iter1 : IteratorWrapper<_> = From::from(vec1.iter_mut());
    let mut vec2 = vec![1,2,3,4];
    let iter2 : IteratorWrapper<_> = From::from(vec2.iter_mut());
    let mut mapped_iter = iter1.chain(iter2).map(|val|{
        Box::new(val)
    });
    while let Some(boxed_obj) = mapped_iter.next() {
        *(*boxed_obj) += 1;
    }
    println!("{:?}", &vec1);
    println!("{:?}", &vec2);
}

pub fn filter() {
    // implement
    let mut vec = vec![1,2,3,4,5];
    let iter : IteratorWrapper<_> = From::from(vec.iter_mut());
    let mut modified = iter.filter(|val|{
        **val % 2 == 1
    }).map(|val|{
        *val += 1;
        val
    });

    while let Some(item) = modified.next() {
        *item += 1;
    }
    println!("{:?}", &vec);
}

pub fn flatten() {
    let vec1 = vec![1,2,3,4,5];
    let vec2 = vec![2,3,4,5,6];
    let vec3 = vec![3,4,5,6,7];
    let vec_of_iters : Vec<IteratorWrapper<_>> = 
    vec![From::from(vec1.iter()), From::from(vec2.iter()), From::from(vec3.iter())];
    let iter : IteratorWrapper<_> = From::from(vec_of_iters.into_iter());
    let mut flattened_iter = iter.flatten();
    while let Some(val) = flattened_iter.next() {
        println!("{}", val);
    }
}

pub fn flat_map() {
    let vec1 = vec![1,2,3,4,5];
    let vec2 = vec![2,3,4,5,6];
    let vec3 = vec![3,4,5,6,7];
    let vec_of_iters : Vec<IteratorWrapper<_>> = 
    vec![From::from(vec1.iter()), From::from(vec2.iter()), From::from(vec3.iter())];
    let iter : IteratorWrapper<_> = From::from(vec_of_iters.into_iter());
    
    let mut flatmap_iter = iter.flat_map(|my_iter|{
        my_iter.map(|val|{
            *val + 1
        })
    });
    
    while let Some(val) = flatmap_iter.next() {
        println!("{}", val);
    }
}

pub fn try_fold() {
    let vec1 = vec![1,2,3,4,5];
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    let res = iter.try_fold(0, |accum, item| {
        Some(accum + *item)
    });
    println!("should be 15: {}", res.unwrap());

    let vec2 = vec!["1", "2", "3", "4", "5"];
    let mut iter : IteratorWrapper<_> = From::from(vec2.iter());
    let res = iter.try_fold(0, |accum, item| {
        let int = (*item).parse::<i32>().ok()?;
        Some(accum + int)
    });
    println!("should be 15: {}", res.unwrap());

    let vec3 = vec!["1", "2", "nil", "4", "5"];
    let mut iter : IteratorWrapper<_> = From::from(vec3.iter());
    let res = iter.try_fold(0, |accum, item| {
        let int = (*item).parse::<i32>().ok()?;
        Some(accum + int)
    });
    println!("should be true: {}", res.is_none());
}

pub fn fold() {
    let vec1 = vec![1,2,3,4,5];
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    let res = iter.fold(0, |accum, item|{
        accum + *item
    });
    println!("{}", res);
}

pub fn count() {
    let vec1 = vec![1,2,3,4,5];
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    let res = iter.count();
    println!("{}", res);

    let vec1 = Vec::<i32>::new();
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    let res = iter.count();
    println!("{}", res);
}

pub fn last() {
    let vec1 = vec![1,2,3,4,5];
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    let res = iter.last().unwrap();
    println!("{}", res);

    let vec1 = Vec::<i32>::new();
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    let res = iter.last();
    println!("{}", res.is_none());
}

pub fn for_each() {
    let vec1 = vec![1,2,3,4,5];
    let mut iter : IteratorWrapper<_> = From::from(vec1.iter());
    iter.for_each(|val| {
        println!("{}", val);
    })
}