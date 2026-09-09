use serde::Serialize;
use std::{collections::HashMap, ops::Rem};
use tracing::info;
use std::ops::{Add, Div};

pub struct PaidIn {
    pub username: String,
    pub amount: f32,
}

#[derive(Debug, PartialEq)]
struct Currency(i64);

impl Currency {
    fn new(value: f32) -> Self {
        let cents = (value * 100.0).round() as i64;
        Currency(cents)
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Currency(self.0 + rhs.0)
    }
}

impl Div for Currency {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Currency(self.0 / rhs.0)
    }
}

impl Rem for Currency {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Currency(self.0 % rhs.0)
    }
}

struct ConvertedPaidIn {
    username: String,
    amount: Currency
}

struct NetOwed {
    fair_share: f32,
    total_amount: f32,
    net_paid_in: HashMap<String, f32>,
}

#[derive(Serialize)]
pub struct PayOut {
    pub fair_share: f32,
    pub total_amount: f32,
    pub transactions: Vec<String>,
}

fn calc_net_owed(paid_in: Vec<ConvertedPaidIn>) -> NetOwed {
    let total_amount: Currency = paid_in.iter().fold(Currency::new(0.0), |acc, x| acc + x.amount);
    
    let fair_share_base = total_amount / Currency::new(paid_in.len() as f32);
    let fair_share_remainder = total_amount % Currency::new(paid_in.len() as f32);
    let net_paid_in: HashMap<String, f32> = paid_in
        .iter()
        .map(|x| (x.username.clone(), x.amount - fair_share))
        .collect();

    info!("total_amount: {total_amount}; fair_share: {fair_share}");
    NetOwed {
        fair_share,
        total_amount,
        net_paid_in,
    }
}

pub fn greedy_min_cash_flow(paid_in: Vec<PaidIn>) -> PayOut {
    let converted: Vec<ConvertedPaidIn> = paid_in.iter().map(|x| ConvertedPaidIn{username: x.username.clone(), amount: Currency::new(x.amount)}).collect();
    let net_owed = calc_net_owed(converted);
    let mut debtors: Vec<(String, f32)> = net_owed
        .net_paid_in
        .iter()
        .filter(|&(_, &net_amount)| net_amount < 0.0)
        .map(|(name, &net_amount)| (name.clone(), -net_amount))
        .collect();

    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut creditors: Vec<(String, f32)> = net_owed
        .net_paid_in
        .iter()
        .filter(|&(_, &net_amount)| net_amount > 0.0)
        .map(|(name, &net_amount)| (name.clone(), net_amount))
        .collect();

    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    debtors.retain(|x| x.1 > 0.01);
    creditors.retain(|x| x.1 > 0.01);

    let mut transactions: Vec<String> = Vec::new();

    while debtors.len() > 0 && creditors.len() > 0 {
        let biggest_creditor = creditors.get(0).expect("none found");
        let biggest_debtor = debtors.get(0).expect("none found");

        let debtor_pays = biggest_creditor.1.min(biggest_debtor.1);
        let biggest_debtor = biggest_debtor.0.clone();
        let biggest_creditor = biggest_creditor.0.clone();
        transactions.push(format!(
            "{biggest_debtor} pays {biggest_creditor} {debtor_pays}"
        ));

        creditors[0].1 -= debtor_pays;
        debtors[0].1 -= debtor_pays;

        debtors.retain(|x| x.1 > 0.01);
        creditors.retain(|x| x.1 > 0.01);

        debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }
    info!("transactions: {transactions:?}");
    PayOut {
        fair_share: net_owed.fair_share,
        total_amount: net_owed.total_amount,
        transactions,
    }
}
