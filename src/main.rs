use std::env;
use std::process;

use minils::Config;

fn main() {
    let mut args = env::args().collect();
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        println!("minils: {}", err);
        process::exit(1);
    });

    minils::run(&config);
}
