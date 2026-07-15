use itertools::Itertools;
use std::{collections::HashMap, iter};


pub trait Report {
    fn generate(&self, map: &HashMap<String, u32>) -> String;
}

pub struct PlainTextReport {
}

impl Report for PlainTextReport {
    fn generate(&self, map: &HashMap<String, u32>) -> String {
        map.keys().sorted().map(
            |key| format!("{}: {}\n", key, map[key])
        ).collect::<String>()
    }
}

pub struct CsvReport {
    pub file_name: String
}

impl Report for CsvReport {
    fn generate(&self, map: &HashMap<String, u32>) -> String {
        // let mut output = String::from("level,count\n");
        // let mut keys: Vec<&String> = map.keys().collect();
        // keys.sort();
        // for key in keys {
        //     output += &format!("{},{}\n", key, map[key]);
        // }
        // output

        iter::once(String::from("level,count\n"))
            .chain(map.keys().sorted().map(|key| format!("{}, {}\n", key, map[key])))
            .collect::<String>()
    }
}

pub fn print_report(reporter: impl Report, map: &HashMap<String, u32>) {
    println!("{}", reporter.generate(map));
}