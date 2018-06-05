
extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate log;
extern crate env_logger;

use dotenv::dotenv;
use std::env;


fn main() {

    info!("starting up");

    dotenv().ok();
    for (key, value) in env::vars() {
        println!(" -- {} : {}", key, value);
    }

    println!("{}", dotenv!("MEANING_OF_LIFE"));
}
