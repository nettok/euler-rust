extern crate core;

use std::env;
use self::core::str::FromStr;

pub enum Input<T> {
    Ok(T),
    Err(String, core::num::ParseIntError)
}

// TODO: generalizar a mas de un argumento y compartir
pub fn get_input_from_args(default: u64) -> Input<u64> {
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