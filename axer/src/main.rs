use std::{fs::File, io::{BufRead, BufReader}};

use regex::Regex;

#[derive(Debug)] 
struct LogEntry {
    date: String,
    pid: usize,
    level: String,
    message: String
}

fn parse_log(raw_log: &str, re: &Regex) -> Option<LogEntry> {
    if let Some(caps) = re.captures(raw_log) {
        let log_entry = LogEntry {
            date: String::from(&caps["timestamp"]),
            pid: caps["pid"].parse::<usize>().unwrap(),
            level: String::from(&caps["level"]),
            message: String::from(&caps["message"]),
        };
        Some(log_entry)
    } else{
        None
    }
    
}

fn main() -> std::io::Result<()>{
    let filepath = "dnf5.log";
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);

    let re = Regex::new(r"^(?P<timestamp>\S+)\s+\[(?P<pid>\d+)\]\s+(?P<level>[A-Z]+)\s+(?P<message>.*)$").unwrap();

    let mut line_count = 0;
    let mut parsed_line_count = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(log_entry) = parse_log(&line, &re) {
            // println!("{:?}", log_entry);
            parsed_line_count += 1;
        } else {
            println!("{}", line);
        }
        
        line_count += 1;
    }
    println!("Number of lines: {}", line_count);
    println!("Number of parsed lines: {}", parsed_line_count);
    Ok(())
}
