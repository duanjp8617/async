#[allow(dead_code)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

#[allow(dead_code)]
pub fn run1() {
    let a = 5;
    let mut generator = || {
        if a > 10 {
            println!("In larger than 10 branch");
            yield 1024;
            println!("Back in larger than 10 branch");
        }
        else {
            println!("In smaller than 10 branch");
        }

        yield 1026;
        println!("Exit");
    };

    for _ in 0..5 {
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(n) => {
                println!("The generator yields {}", n);
            },
            _ => {
                println!("The generator exits");
                break;
            },
        }
    }
}

#[allow(dead_code)]
pub fn run2() {
    enum GeneratorImpl {
        Enter(i32),
        Phase1(i32),
        Phase2(i32),
        Exit(()),
    }


    // generating generator code requires control flow analysis
    impl Generator<()> for GeneratorImpl {
        type Yield = i32;
        
        type Return = ();

        fn resume(self: Pin<&mut Self>, _: ()) -> GeneratorState<Self::Yield, Self::Return> {
            let inner = unsafe {self.get_unchecked_mut()};
            match std::mem::replace(inner, GeneratorImpl::Enter(0)) {
                GeneratorImpl::Enter(a) => {
                    if a > 10 {
                        println!("In larger than 10 branch");
                        *inner = GeneratorImpl::Phase1(1024);
                        GeneratorState::Yielded(1024)
                    }
                    else {
                        println!("In smaller than 10 branch");
                        *inner = GeneratorImpl::Phase2(1026);
                        GeneratorState::Yielded(1026)
                    }
                },
                GeneratorImpl::Phase1(_) => {
                    println!("Back in larger than 10 branch");
                    *inner = GeneratorImpl::Phase2(1026);
                    GeneratorState::Yielded(1026)
                },
                GeneratorImpl::Phase2(_) => {
                    println!("Exit");
                    *inner = GeneratorImpl::Exit(());
                    GeneratorState::Complete(())
                },
                GeneratorImpl::Exit(_) => {
                    panic!("Calling finished generator")
                }
            }
        }
    }

    let a = 5;
    let mut gen = GeneratorImpl::Enter(a);
    
    for _ in 0..5 {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(n) => {
                println!("The generator yielded {}", n);
            },
            _ => {
                println!("The generator exists");
                break;
            }
        }
    }
}