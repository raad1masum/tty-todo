extern crate clap;
use clap::{Arg, App};

fn main() {
    let _matches = App::new("tty-todo")
        .version("1.0")
        .author("Raadwan Masum <piraadwan@gmail.com>")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    println!("Hello, World!");
}
