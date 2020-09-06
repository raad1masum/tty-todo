extern crate clap;
use clap::{Arg, App};
use rustop::opts;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write, BufRead};

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
            add_task(formatted_task);
            //let mut file = OpenOptions::new()
            //.write(true)
            //.append(true)
            //.open("/tmp/todo")
            //.unwrap();
            //if let Err(e) = writeln!(file, "{}", formatted_task) {
            //eprintln!("Couldn't write to file: {}", e);
            //}
        }
    }

    if args.complete {
        println!("Complete task");
        if let Some(ref task) = args.task { println!("{}", task); }
        append_file().expect("Not found");
    }

    if args.delete {
        println!("Delete task");
        if let Some(ref task) = args.task { println!("{}", task); }
    }

    if args.list {
        println!("List tasks");
        let mut file = File::open("/tmp/todo").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        println!("{}", contents);
    }
}

fn add_task(task: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/todo")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", task) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn append_file() -> io::Result<()> {
    let file = File::open("/tmp/todo").expect("Unable to open");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
