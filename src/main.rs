extern crate clap;

use clap::{App, Arg};
use rustop::opts;
use std::fs::{write, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

static STORE_FILE: &str = "/tmp/todo";

fn main() {
    init();

    let _matches = App::new("tty-todo")
        .version("1.0")
        .author("Raadwan Masum <piraadwan@gmail.com>")
        .arg(Arg::with_name("add")
                .long("add")
                .short("a")
                .multiple(true)
                .help("Add task"),
        )
        .arg(Arg::with_name("delete")
                .long("delete")
                .short("d")
                .multiple(true)
                .help("Delete task"),
        )
        .arg(Arg::with_name("complete")
                .long("complete")
                .short("c")
                .multiple(true)
                .help("Complete task"),
        )
        .arg(Arg::with_name("list")
                .long("list")
                .short("l")
                .multiple(true)
                .help("List tasks"),
        )
        .arg(Arg::with_name("TASK")
                .help("Task entry")
                .required(false)
                .index(1),
        )
        .get_matches();

    let (args, _rest) = opts! {
        opt add:bool, desc:"Add task";
        opt complete:bool, desc:"Complete task";
        opt delete:bool, desc:"Delete task";
        opt list:bool, desc:"List tasks";
        param task:Option<String>, desc:"Task";
    }
    .parse_or_exit();

    if args.add {
        if let Some(ref task) = args.task {
            println!("{}", task);
            let formatted_task: String = "[ ] ".to_owned() + task;
            add_task(formatted_task);
        }
    }

    if args.complete {
        if let Some(ref task) = args.task {
            let formatted_task: String = "[ ] ".to_owned() + task;
            complete_task(formatted_task).expect("Not found");
        }
    }

    if args.delete {
        if let Some(ref task) = args.task {
            let formatted_task: String = "[ ] ".to_owned() + task;
            let formatted_complete_task: String = "[x] ".to_owned() + task;
            delete_task(formatted_task, formatted_complete_task).expect("Not found");
        }
    }

    if args.list {
        println!("TODO:");
        list_tasks();
    }
}

fn init() {
    if !Path::new("/tmp/todo").exists() {
        let mut _file = File::create("/tmp/todo");
    }
}

fn add_task(task: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(STORE_FILE)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", task) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn complete_task(task: String) -> io::Result<()> {
    let file = File::open(STORE_FILE).expect("Unable to open");
    let reader = BufReader::new(file);
    let mut task_list = Vec::new();
    for line in reader.lines() {
        task_list.push(line?.to_string());
    }
    for i in 0..task_list.len() {
        if task_list[i] == task {
            task_list[i] = task_list[i].replace("[ ]", "[x]");
        }
    }
    write(STORE_FILE, "").expect("Unable to write file");
    for i in 0..task_list.len() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(STORE_FILE)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", &task_list[i]) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    Ok(())
}

fn delete_task(task: String, complete_task: String) -> io::Result<()> {
    let file = File::open(STORE_FILE).expect("Unable to open");
    let reader = BufReader::new(file);
    let mut task_list = Vec::new();
    for line in reader.lines() {
        task_list.push(line?.to_string());
    }
    for i in 0..task_list.len() {
        if task_list[i] == task || task_list[i] == complete_task {
            task_list.remove(i);
        }
    }
    write(STORE_FILE, "").expect("Unable to write file");
    for i in 0..task_list.len() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(STORE_FILE)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", &task_list[i]) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    Ok(())
}

fn list_tasks() {
    let mut file = File::open(STORE_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    println!("{}", contents);
}
