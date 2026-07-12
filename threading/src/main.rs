use std::thread::{self, JoinHandle};
use std::sync::mpsc::channel;

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use std::sync::Arc;
use regex::Regex;

#[derive(Debug)]
enum AppError {
    IOError(std::io::Error),
    ReError(regex::Error)
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IOError(e)
    }
}

impl From<regex::Error> for AppError {
    fn from(e: regex::Error) -> Self {
        AppError::ReError(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IOError(err) => write!(f, "IO Error: {}", err),
            AppError::ReError(err) => write!(f, "Regex Error: {}", err),
        }
    }
}


trait Report {
    fn generate(&self, map: &HashMap<String, u32>) -> String;
}

struct PlainTextReport {
}

impl Report for PlainTextReport {
    fn generate(&self, map: &HashMap<String, u32>) -> String {
        let mut output = String::new();
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();
        for key in keys {
            output += &format!("{}: {}\n", key, map[key]);
        }
        output
    }
}

struct CsvReport {
    file_name: String
}

impl Report for CsvReport {
    fn generate(&self, map: &HashMap<String, u32>) -> String {
        let mut output = String::from("level,count\n");
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();
        for key in keys {
            output += &format!("{},{}\n", key, map[key]);
        }
        output
    }
}

fn print_report(reporter: impl Report, map: &HashMap<String, u32>) {
    println!("{}", reporter.generate(&map));
}

fn parse_log(raw_log: &str, re: &Regex) -> Option<String> {
    if let Some(caps) = re.captures(raw_log) {
        Some(String::from(&caps["level"]))
    } else{
        None
    }
    
}

fn count_level(chunk: &[String], re: &Regex, map: &mut HashMap<String, u32>) {
    for line in chunk {
        if let Some(log_level) = parse_log(line, re) {
            *map.entry(log_level).or_insert(0) += 1;
        }
    }
}


fn main() -> Result<(), AppError>{
    let filepath = "dnf5.log";
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);

    let re = Arc::new(Regex::new(r"^(?P<timestamp>\S+)\s+\[(?P<pid>\d+)\]\s+(?P<level>[A-Z]+)\s+(?P<message>.*)$")?);
    let mut log_counter: HashMap<String, u32> = HashMap::new();

    // read all lines into vec string
    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        lines.push(line?);
    }

    // num threads & chunks
    let num_threads = 2;
    let chunk_size = lines.len() / num_threads;

    // create channel
    let (tx, rx) = channel();
    
    // spawn threads for chunks
    let lines = Arc::new(lines);
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for thread_idx in 0..num_threads {
        let tx_copy = tx.clone();
        let re_copy = re.clone();
        let lines_copy = Arc::clone(&lines);
        let thread = thread::spawn(move || {
            let mut hash : HashMap<String, u32> = HashMap::new();
            let start_idx = thread_idx * chunk_size;
            count_level(&lines_copy[start_idx .. start_idx + chunk_size], &re_copy, &mut hash);
            tx_copy.send(hash).expect("Unable to send on channel");
        });
        threads.push(thread);
    }

    for _thread_idx in 0..num_threads {
        let received_map = rx.recv().expect("Unable to receive from channel");
        for (log_message, count) in received_map {
            *log_counter.entry(log_message).or_insert(0) += count;
        }
    }

    for thread in threads {
        thread.join().expect("The thread has panicked closing");
    }


    print_report(CsvReport{file_name: String::from("filename.csv")}, &log_counter);
    
    Ok(())
}
