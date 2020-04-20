#[allow(dead_code)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

#[allow(dead_code)]
pub fn run() {
    enum GeneratorImpl {
        Enter(i32),
        Phase1(i32),
        Phase2(i32),
        Exit(()),
    }

    impl Generator<()> for GeneratorImpl {
        type Yield = i32;
        
        type Return = ();

        fn resume(self: Pin<&mut Self>, _: ()) -> GeneratorState<Self::Yield, Self::Return> {
            let inner = unsafe {self.get_unchecked_mut()};
            match std::mem::replace(inner, GeneratorImpl::Enter(0)) {
                GeneratorImpl::Enter(a) => {
                    println!("Hello");
                    *inner = GeneratorImpl::Phase1(4 * a);
                    GeneratorState::Yielded(4*a)
                },
                GeneratorImpl::Phase1(_) => {
                    println!("Hello"); 
                    *inner = GeneratorImpl::Phase2(777);
                    GeneratorState::Yielded(777)
                },
                GeneratorImpl::Phase2(_) => {
                    println!("Hello");
                    *inner = GeneratorImpl::Exit(());
                    GeneratorState::Complete(())
                },
                GeneratorImpl::Exit(_) => {
                    *inner = GeneratorImpl::Exit(());
                    GeneratorState::Complete(())
                }
            }
        }
    }

    let a = 32;
    let mut gen = GeneratorImpl::Enter(a);
    
    for _ in 0..5 {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(n) => {
                println!("The generator yielded {}", n);
            },
            _ => {
                println!("The generator completes");
            }
        }
    }
}