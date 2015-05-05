#![feature(core)]

mod input;
use input::{Input, get_input_from_args};

mod p001;
use p001::{imperative, declarative};

mod p002;

fn main() {
    let below = match get_input_from_args().unwrap_or(Input::Ok(1000)) {
        Input::Ok(x) => x,
        Input::Err(arg, err) => {
            println!("Numero invalido: {}.  Error: {:?}", arg, err);
            return;
        }
    };

    println!("imperative: {}", imperative(below));
    println!("declarative: {}", declarative(below));
}
