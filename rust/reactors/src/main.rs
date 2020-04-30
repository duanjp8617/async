// This repo contains a series of reactor implementations. 
// I'm writing these code to explore the essence of 
// Rust async-await. Back in 2017, when I was working on the 
// Netstar project, I only read the source code of Seastar. 
// The future/promise implementation of Seastar is so damn hard.
// I only know roughly how it works, but I can't write a similar
// future/promise implementation by myself.

// Compared to Seastar, the Rust future implementation is extremely
// well documented and easy to understand. I never thought that I
// would have the chance to find such a simple and elegament implementation.
// I'd like to take this valuable opportunity and carefully study every
// aspects of the Rust future-based programming system. And I believe 
// the efforts dedicated to this exploration will not vanish in vain:
// I'm planning to do a systematic comparison between Rust's async
// programming system with C/C++'s async programming system, this may
// lead to a very good paper; I'm also planning to build a high-performance
// user-space operating system that supports fast user-space transport 
// stack, this needs solid background knowledge on how Rust's future
// works, and may also lead to a good paper getting published.

// My goal is to carefully study all the implementation details of 
// Rust future implementation, and re-implement most of the core 
// features by myself. The resulting code can be a very good tutorial
// for me. And maybe others can learn from it.

// Reactor1:
// A single-thread reactor motivated by fahrenheit, that only supports 
// async sleep.  
mod reactor1;
mod reactor2;

fn main() {
    reactor2::launch();
}
