#![feature(core)]

mod input;
use input::{Input, get_input_from_args};

mod p0001;
use p0001::{imperative, declarative};

fn main() {
    let below = match get_input_from_args(1000) {
        Input::Ok(x) => x,
        Input::Err(arg, err) => {
            println!("Numero invalido: {}.  Error: {:?}", arg, err);
            return;
        }
    };

    println!("imperative: {}", imperative(below));
    println!("declarative: {}", declarative(below));
}
