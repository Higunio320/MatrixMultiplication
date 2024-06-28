mod matrix;
mod multiplication;

use std::{env, process};
use multiplication::Config;
use crate::multiplication::run;

fn main() {
    let config = Config::from_iter(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem passing arguments:\n{}", err);
            process::exit(1);
        });

    match run(config) {
        Ok(_) => println!("Success!"),
        Err(error) => {
            eprintln!("Application error:\n{}", error);
            process::exit(1);
        }
    }
}
