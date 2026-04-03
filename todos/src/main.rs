use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const TASK_FILEPATH: &'static str = "tasks.txt";

fn add_tasks(tasks: Vec<String>) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(TASK_FILEPATH)
        .expect("myah");
    
    for task in tasks {
        let task = task.as_str();
        writeln!(file,"{}", task).expect("myaa");
    }
    
}

fn complete_tasks() {
    
}

fn list_tasks() {
    let file = File::open(TASK_FILEPATH).expect("msg");
    let reader = BufReader::new(file);
    
    for (index, line) in reader.lines().enumerate(){
        println!("{}. {}", index, line.expect("blah"));
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.len() > 1 {
        match args[0].as_str() {
            "add" => add_tasks(args[1..].to_vec()),
            "done" => println!("done me"),
            _ => println!("not")
        }
    } else {
        list_tasks();
    }
}
