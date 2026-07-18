
use std::thread::{self, JoinHandle};
use std::sync::mpsc::channel;
use clap::{Parser, ValueEnum};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use std::sync::Arc;
use regex::Regex;
use std::path::PathBuf;


mod errors;
mod report;
mod parser;

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    Csv,
    Text,
}

#[derive(Parser)]
struct Cli {
    #[arg(long, value_enum, default_value_t = Format::Text)]
    format: Format,
    #[arg(long, default_value = "dnf5.log")]
    file: PathBuf,
}

fn main() -> Result<(), errors::AppError>{
    let args = Cli::parse();

    let f = File::open(&args.file)?;
    let reader = BufReader::new(f);

    let re = Arc::new(Regex::new(r"^(?P<timestamp>\S+)\s+\[(?P<pid>\d+)\]\s+(?P<level>[A-Z]+)\s+(?P<message>.*)$")?);

    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, std::io::Error>>()?;

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
            parser::count_level(&lines_copy[start_idx .. start_idx + chunk_size], &re_copy, &mut hash);
            tx_copy.send(hash).expect("Unable to send on channel");
        });
        threads.push(thread);
    }

    let log_counter: HashMap<String, u32> = (0..num_threads).fold(HashMap::new(), |mut acc, _| {
        let received_map = rx.recv().expect("Unable to receive from channel");
        for (log_message, count) in received_map.into_iter() {
            *acc.entry(log_message.clone()).or_insert(0) += count;
        }
        acc
    });

    for thread in threads {
        thread.join().expect("The thread has panicked closing");
    }

    match args.format {
        Format::Csv => report::print_report(&report::CsvReport{file_name: String::from("filename.csv")}, &log_counter),
        Format::Text => report::print_report(&report::PlainTextReport{}, &log_counter),
    }
    
    Ok(())
}
