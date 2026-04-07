use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const TASK_FILEPATH: &'static str = "tasks.txt";

fn add_tasks(task: Vec<String>) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(TASK_FILEPATH)
        .expect("failed opening file");
    
    
    let task = task.join(" ");
    writeln!(file,"☐ {}", task).expect("failed writing file");

    println!("Finished adding task")
}


fn complete_tasks(task_numbers: Vec<String>) {
    let mut conv: Vec<usize> = vec![];
    for task_number in task_numbers {
        let task_number: usize = task_number.parse().expect("Not a valid number");
        conv.push(task_number);
    }
    
    let content = fs::read_to_string(TASK_FILEPATH).expect("could not read file");
    
    let new_lines: Vec<String> = content
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            if conv.contains(&idx) && line.starts_with('☐') {
                line.replace('☐', "✓")
            } else {
                line.to_string()
            }
        })
        .collect();
    
    let output = new_lines.join("\n");
    fs::write(TASK_FILEPATH, output).expect("could not write to file");
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
            "done" => complete_tasks(args[1..].to_vec()),
            _ => println!("not")
        }
        list_tasks();
    } else {
        list_tasks();
    }
}
