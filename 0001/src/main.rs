// Multiples of 3 and 5
// Problem 1

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

#![feature(core)]

extern crate core;

use std::env;
use core::str::FromStr;

fn imperative(below: u64) -> u64 {
    let mut sum = 0;

    for x in 3..below {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    sum
}

fn declarative(below: u64) -> u64 {
    (3..below)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

enum Input<T> {
    Ok(T),
    Err(String, core::num::ParseIntError)
}

// TODO: generalizar a mas de un argumento y compartir
fn get_input_from_args(default: u64) -> Input<u64> {
    let args: Vec<String> = env::args().collect();

    return
        if args.len() < 2 {
            Input::Ok(default)
        }
        else {
            match u64::from_str(&args[1]) {
                Ok(x) => Input::Ok(x),
                Err(err) => Input::Err(args[1].clone(), err)
            }
        };
}

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
