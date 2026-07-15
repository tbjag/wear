use regex::Regex;
use std::{collections::HashMap};

fn parse_log(raw_log: &str, re: &Regex) -> Option<String> {
    re.captures(raw_log).map(|caps| String::from(&caps["level"]))
}

pub fn count_level(chunk: &[String], re: &Regex, map: &mut HashMap<String, u32>) {
    chunk.iter()
        .filter_map(|line| parse_log(line, re))
        .for_each(|log_level| *map.entry(log_level).or_insert(0) += 1)
}