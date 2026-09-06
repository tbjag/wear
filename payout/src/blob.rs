

fn get_amounts() -> HashMap<String, f32> {
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
    return paid_in;
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
    net_paid_in
}

fn greedy_min_cash_flow(net_owed: HashMap<String, f32>) {
    let mut debtors: Vec<(String, f32)> = net_owed
        .iter()
        .filter(|&(_, &net_amount)| net_amount < 0.0)
        .map(|(name, &net_amount)| (name.clone(), -net_amount))
        .collect();

    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut creditors: Vec<(String, f32)> = net_owed
        .iter()
        .filter(|&(_, &net_amount)| net_amount > 0.0)
        .map(|(name, &net_amount)| (name.clone(), net_amount))
        .collect();

    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    debtors.retain(|x| x.1 > 0.01);
    creditors.retain(|x| x.1 > 0.01);

    while debtors.len() > 0 && creditors.len() > 0 {
        let biggest_creditor = creditors.get(0).expect("none found");
        let biggest_debtor = debtors.get(0).expect("none found");

        let debtor_pays = biggest_creditor.1.min(biggest_debtor.1);
        let biggest_debtor = biggest_debtor.0.clone();
        let biggest_creditor = biggest_creditor.0.clone();
        println!("{biggest_debtor} pays {biggest_creditor} {debtor_pays}");

        creditors[0].1 -= debtor_pays;
        debtors[0].1 -= debtor_pays;

        debtors.retain(|x| x.1 > 0.01);
        creditors.retain(|x| x.1 > 0.01);

        debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }

    println!("DONE");
}

// let paid_in = get_amounts();
// let net_owed = calc_net_owed(paid_in);
// greedy_min_cash_flow(net_owed);