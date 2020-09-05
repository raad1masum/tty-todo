extern crate clap;
use clap::{Arg, App};
use rustop::opts;
use std::fs::File;
use std::fs::write;
use std::io::{Write};

fn main() {
    let _matches = App::new("tty-todo")
        .version("1.0")
        .author("Raadwan Masum <piraadwan@gmail.com>")
        .arg(Arg::with_name("add")
            .long("add")
            .short("a")
            .multiple(true)
            .help("Add task"))
        .arg(Arg::with_name("delete")
            .long("delete")
            .short("d")
            .multiple(true)
            .help("Delete task"))
        .arg(Arg::with_name("complete")
            .long("complete")
            .short("c")
            .multiple(true)
            .help("Complete task"))
        .arg(Arg::with_name("list")
            .long("list")
            .short("l")
            .multiple(true)
            .help("List tasks"))
        .arg(Arg::with_name("TASK")
            .help("Task entry")
            .required(false)
            .index(1))
        .get_matches();

    let (args, _rest) = opts! {
        opt add:bool, desc:"Add task";
        opt complete:bool, desc:"Complete task";
        opt delete:bool, desc:"Delete task";
        opt list:bool, desc:"List tasks";
        param task:Option<String>, desc:"Task";
    }.parse_or_exit();

    if args.add {
        println!("Add task");
        if let Some(ref task) = args.task { 
            println!("{}", task); 
            let formatted_task: String = "[ ] ".to_owned() + task;
            write("/tmp/todo", formatted_task).expect("Unable to write file");
        }
    }

    if args.complete {
        println!("Complete task");
        if let Some(ref task) = args.task { println!("{}", task); }
    }

    if args.delete {
        println!("Delete task");
        if let Some(ref task) = args.task { println!("{}", task); }
    }

    if args.list {
        println!("List tasks");
    }
}
