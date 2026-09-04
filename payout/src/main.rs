use std::{collections::HashMap, io::{self, Write}};
// use clap::{Parser};

// #[derive(Parser)]
// #[command(name = "payout", version = "0.0")]
// struct Cli {
//     #[arg(long)]
//     new: bool,
//     // #[arg(short, long, default_value_t=1)]
//     // count: u8
// }

fn get_amounts()-> HashMap<String, f32> {
    let mut names: Vec<String> = Vec::new();
    println!("Enter all of the names of the party: (`hit enter` to continue)");
    io::stdout().flush().unwrap();
    
    loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let name = input.trim();
        if name.is_empty() {
            break;
        }
        names.push(name.to_string());
    }

    let mut paid_in: HashMap<String, f32> = HashMap::new();
    println!("Enter expenses by person: (`next` to continue)");
    
    
    for person in names {
        print!("{person}: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let amount = input.trim();
        let parsed_amount: f32 = amount.parse().expect("could not parse into usize");
        paid_in.insert(person.clone(), parsed_amount);

    }
    println!("{paid_in:?}");
    return  paid_in;
}

fn calc_net_owed(paid_in: HashMap<String, f32>) -> HashMap<String, f32> {
    let total_amount: f32 = paid_in.values().copied().sum();
    let fair_share = total_amount / paid_in.len() as f32;
    println!("fair_share: {fair_share}");
    let net_paid_in: HashMap<String, f32> = paid_in
        .iter()
        .map(|(person, amount)| (person.clone(), amount - fair_share))
        .collect();
    println!("{net_paid_in:?}");
    paid_in
}

fn greedy_min_cash_flow(net_paid_in: HashMap<String, f32>) {
    
}

fn main() {
    // let cli = Cli::parse();
    // if cli.new {
    //     println!("new")
    // }

    // get stdin of:
    // ppl
    // how much each paid

    let paid_in = get_amounts();
    let net_owed= calc_net_owed(paid_in);

    
    
}