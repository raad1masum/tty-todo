extern crate clap;
use clap::{Arg, App};

fn main() {
    let _matches = App::new("tty-todo")
        .version("1.0")
        .author("Raadwan Masum <piraadwan@gmail.com>")
        .arg(Arg::with_name("INPUT")
            .help("Task entry")
            .required(true)
            .index(1))
        .arg(Arg::with_name("a")
            .short("a")
            .multiple(true)
            .help("Add task"))
        .arg(Arg::with_name("d")
            .short("d")
            .multiple(true)
            .help("Delete task"))
        .get_matches();
    println!("Hello, World!");
}
