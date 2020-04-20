#[allow(dead_code)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

#[allow(dead_code)]
pub fn run() {
    let a = 32;
    let mut generator = || {
        println!("Hello");
        yield a*4;
        println!("Hello"); 
        yield 777;
        println!("Hello");
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(n) => {
            println!("shit happens with {}", n);
        },
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(777) => {
            println!("shit happens ");
        },
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete(()) => {
            println!("shit happens again");
        }
        _ => panic!("unexpected return from resume"),
    }
}