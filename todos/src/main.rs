use std::{env, usize};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use anyhow::Result;

const TASK_FILEPATH: &str = "tasks.txt";

fn add_tasks(task: &[String]) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(TASK_FILEPATH)?;

    let task = task.join(" ");
    writeln!(file, "☐ {}", task)?;

    println!("Finished adding task");
    Ok(())
}

fn complete_tasks(task_numbers: &[String]) -> Result<()>{
    let conv: Vec<usize> = task_numbers
        .iter()
        .map(|x| x.parse::<usize>().map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;

    let content = fs::read_to_string(TASK_FILEPATH)?;

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

    let output = new_lines.join("\n") + "\n";
    fs::write(TASK_FILEPATH, output)?;
    Ok(())
}

fn list_tasks() -> Result<()>{
    let file = File::open(TASK_FILEPATH)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{}. {}", index, line);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        list_tasks()?;
    } else {
        match args[0].as_str() {
            "add" if args.len() > 1 => {
                add_tasks(&args[1..])?;
                list_tasks()?;
            }
            "done" if args.len() > 1 => {
                complete_tasks(&args[1..])?;
                list_tasks()?;
            }
            "add" | "done" => eprintln!("missing arguments"),
            _ => eprintln!("option not available"),
        }
    }
    
    Ok(())
}
