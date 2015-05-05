extern crate core;

use std::env;
use self::core::str::FromStr;

pub enum Input<T> {
    Ok(T),
    Err(String, core::num::ParseIntError)
}

// TODO: generalizar a mas de un argumento
pub fn get_input_from_args() -> Option<Input<usize>> {
    let args: Vec<String> = env::args().collect();

    return
        if args.len() < 2 {
            None
        }
        else {
            match usize::from_str(&args[1]) {
                Ok(x) => Some(Input::Ok(x)),
                Err(err) => Some(Input::Err(args[1].clone(), err))
            }
        };
}