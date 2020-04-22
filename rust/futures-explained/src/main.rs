#![feature(generators, generator_trait)]

mod play_with_threads;
mod callback;
mod trait_object;

mod generator1;
mod generator2;
mod generator3;

mod pin1;
mod pin2;
mod pin3;

mod future_executor1;

fn main() {
    future_executor1::run();
}
