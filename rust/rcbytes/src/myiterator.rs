use std::ops::Try;

pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn nth(&mut self, mut n : usize) -> Option<Self::Item> {
        while let item @ Some(..) = self.next() {
            if n == 0 {
                return item
            }
            n -= 1;
        }
        None

        // The stdlib implementation requires that 
        // &mut I to be an iterator, which is hard to 
        // achieve with a customized implementation
        // for x in self {
        //     if n == 0 {
        //         return Some(x)
        //     }
        //     n -= 1
        // }
        // None
    }

    // stdlib makes U a subtype of IntoIterator, making 
    // the API more flexibly
    fn chain<U>(self, other : U) -> MyChain<Self, U>
    where Self : Sized, U : MyIterator<Item = Self::Item> {
        MyChain::new(self, other)
    }

    fn map<F, R>(self, f : F) -> MyMap<Self, F> 
    where Self : Sized, F : FnMut(Self::Item) -> R {
        MyMap::new(self, f)
    }

    fn filter<F>(self, f : F) -> MyFilter<Self, F>
    where Self : Sized, F : FnMut(&Self::Item) -> bool {
        MyFilter::new(self, f)
    }

    fn flatten(self) -> MyFlatten<Self::Item, Self>
    where Self : Sized, Self::Item : MyIterator {
        MyFlatten::new(self)
    }

    // option 1: put the type tag (MyMap) in the public API
    // fn flat_map<F, R>(self, f : F) -> MyFlatMap<R, MyMap<Self, F>>
    // where Self : Sized, Self::Item : MyIterator, R : MyIterator, F : FnMut(Self::Item) -> R,
    // {
    //     MyFlatMap::new(self.map(f).flatten())
    // }
    
    // option 2: put the type tag (MyMap) in new.. well, it doesn't change anything
    // fn flat_map<F, R>(self, f : F) -> MyFlatMap<R, MyMap<Self, F>>
    // where Self : Sized, Self::Item : MyIterator, R : MyIterator, F : FnMut(Self::Item) -> R,
    // {
    //     MyFlatMap::new(self, f)
    // }
    
    // option 3: change the definition of MyFlatMap
    fn flat_map<F, R>(self, f : F) -> MyFlatMap<Self, F, R>
    where Self : Sized, Self::Item : MyIterator, R : MyIterator, F : FnMut(Self::Item) -> R,
    {
        MyFlatMap::new(self, f)
    }

    fn try_fold<F, R, A>(&mut self, a : A, mut f : F) -> R
    where 
        F : FnMut(A, Self::Item) -> R, 
        R : Try<Ok = A> 
    {
        let mut accum = a;
        while let Some(item) = self.next() {
            accum = f(accum, item)?;
        }
        Try::from_ok(accum)
    }

    fn fold<F, A>(&mut self, a : A, f : F) -> A
    where 
        F : FnMut(A, Self::Item) -> A 
    {
        #[inline]
        fn call<A, Item>(mut f : impl FnMut(A, Item) -> A) -> impl FnMut(A, Item) -> Result<A, !> {
            move |accum, item| {
                Ok(f(accum, item))
            }
        }

        self.try_fold(a, call(f)).unwrap()
    }

    fn count(&mut self) -> usize {
        // self.fold(0, |accum, _| {accum + 1})
        #[inline]
        fn fold_func<Item>() -> impl FnMut(usize, Item) -> Result<usize, !> {
            |accum, _| {
                Ok(accum + 1)
            }
        }

        self.try_fold(0, fold_func()).unwrap()
    }

    fn last(&mut self) -> Option<Self::Item> {
        // option 1
        // let init = self.next();
        // if init.is_none() {
        //     init
        // }
        // else {
        //     self.fold(init, |_, item| {
        //         Some(item)
        //     })
        // }

        // option 2 : a simplified version of version 1
        // self.next().map(|init|{
        //     self.fold(init, |_, item|{
        //         item
        //     })
        // })

        // option 3 : convert option 2 to try_fold
        #[inline]
        fn fold_func<Item>() -> impl FnMut(Item, Item) -> Result<Item, !> {
            |_, item| {
                Ok(item)
            } 
        }

        self.next().map(|init| {
            self.try_fold(init, fold_func()).unwrap()
        })
    }

    fn for_each<F>(&mut self, f : F)
    where 
        F : FnMut(Self::Item) 
    {
        fn fold_func<Item>(mut f : impl FnMut(Item)) -> impl FnMut((), Item) -> Result<(), !> {
            move |_, item| {
                Ok(f(item))
            }
        }
        self.try_fold((), fold_func(f)).unwrap()
    }
}

enum MyChainState {
    IteratingA,
    IteratingB,
}

// chain-related helper structures
pub struct MyChain<A, B> {
    a : A,
    b : B,
    state : MyChainState, // 0 for a, 1 for b    
}

impl<A, B> MyChain<A, B> {
    fn new(a : A, b : B) -> Self {
        MyChain {
            a : a,
            b : b,
            state : MyChainState::IteratingA,
        }
    }
}

impl<A, B> MyIterator for MyChain<A, B> 
where A : MyIterator, B : MyIterator<Item = A::Item> {
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            MyChainState::IteratingA => {
                match self.a.next() {
                    res @ Some(..) => res,
                    None => {
                        self.state = MyChainState::IteratingB;
                        self.b.next()
                    }
                }
            },
            MyChainState::IteratingB => {
                self.b.next()
            }
        }
    }
}

// map-related helper structures
pub struct MyMap<I, F> {
    iter : I,
    f : F,
}

impl<I, F> MyMap<I, F> {
    fn new(iter : I, f : F) -> Self {
        MyMap {
            iter : iter,
            f : f,
        }
    }
}

impl<I, F, R> MyIterator for MyMap<I, F>
where I : MyIterator, F : FnMut(I::Item) -> R {
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        // self.iter.next().map(|item| {
        //     (self.f)(item)
        // })
        
        // The stdlib version is way better
        self.iter.next().map(&mut self.f)
    }
}

// filter-related helper structures
pub struct MyFilter<I, F> {
    iter : I,
    f : F,
}

impl<I, F> MyFilter<I, F> {
    fn new(iter : I, f : F) -> Self {
        MyFilter {
            iter : iter,
            f : f,
        }
    }
}

impl<I, F> MyIterator for MyFilter<I, F> 
where I : MyIterator, F : FnMut(&I::Item) -> bool {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.f)(&item) {
                return Some(item)
            }
        }
        return None
    }
} 

// flatten-related helper structures
pub struct MyFlatten<I, II> {
    curr : Option<I>,
    iters : II,
}

impl<I, II> MyFlatten<I, II>
where I : MyIterator, II : MyIterator<Item = I> {
    fn new(mut iters : II) -> Self {
        let item = iters.next();
        MyFlatten {
            curr : item,
            iters : iters
        }
    }
}

impl<I, II> MyIterator for MyFlatten<I, II>
where I : MyIterator, II : MyIterator<Item = I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr.is_some() {
            let item = self.curr.as_mut().unwrap().next();
            if item.is_some() {
                return item
            }
            else {
                self.curr = self.iters.next();
            }
        };
        None
    }
}

// flatmap-related helper structures

// option1: put the type tag (MyMap) in the public API
// pub struct MyFlatMap<I, II> {
//     inner : MyFlatten<I, II>,
// }

// impl<I, II> MyFlatMap<I, II> {
//     fn new(inner : MyFlatten<I, II>) -> Self {
//         MyFlatMap {
//             inner : inner
//         }
//     }
// }

// impl<I, II> MyIterator for MyFlatMap<I, II>
// where I : MyIterator, II : MyIterator<Item = I> {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.next()
//     }
// }


// option 2: put the type tag (MyMap) in new.. well, it doesn't change anything
// pub struct MyFlatMap<I, II> {
//     inner : MyFlatten<I, II>,
// }

// impl<F, R, I, II> MyFlatMap<R, MyMap<II, F>>
// where I : MyIterator, R : MyIterator, F : FnMut(I) -> R, II : MyIterator<Item = I>
// {
//     fn new(iters : II, f : F) -> Self{
//         let my_flatten = iters.map(f).flatten();
//         MyFlatMap {
//             inner : my_flatten
//         }
//     }
// }

// impl<I, II> MyIterator for MyFlatMap<I, II>
// where I : MyIterator, II : MyIterator<Item = I> {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.next()
//     }
// }

// option 3: change the definition of MyFlatMap
pub struct MyFlatMap<II, F, R> {
    inner : MyFlatten<R, MyMap<II, F>>,
}

impl<F, R, I, II> MyFlatMap<II, F, R> 
where I : MyIterator, R : MyIterator, F : FnMut(I) -> R, II : MyIterator<Item = I> 
{
    fn new(iters : II, f : F) -> Self {
        let my_flatten = iters.map(f).flatten();
        MyFlatMap {
            inner : my_flatten
        }
    }
}

impl<F, R, I, II> MyIterator for MyFlatMap<II, F, R>
where I : MyIterator, II : MyIterator<Item = I>, F : FnMut(I) -> R, R : MyIterator {
    type Item = R::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}