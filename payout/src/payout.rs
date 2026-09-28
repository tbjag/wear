use serde::Serialize;
use std::{collections::HashMap};
use tracing::info;
use crate::currency::Currency;

pub struct PaidIn {
    pub username: String,
    pub amount: f32,
}

struct ConvertedPaidIn {
    username: String,
    amount: Currency
}

struct NetOwed {
    fair_share: Currency,
    total_amount: Currency,
    net_paid_in: HashMap<String, Currency>,
}

#[derive(Serialize)]
pub struct PayOut {
    pub fair_share: String,
    pub total_amount: String,
    pub transactions: Vec<String>,
}

fn calc_net_owed(paid_in: Vec<ConvertedPaidIn>) -> NetOwed {
    let total_amount: Currency = paid_in.iter().fold(Currency::new(0.0), |acc, x| acc + x.amount);
    let fair_share_base = total_amount / paid_in.len() as i64;
    let fair_share_remainder = total_amount % paid_in.len() as i64;
    let net_paid_in: HashMap<String, Currency> = paid_in
        .iter()
        .map(|x| (x.username.clone(), x.amount - fair_share_base))
        .collect();
    // todo!("add remainder");

    info!("total_amount: {total_amount}; fair_share_base: {fair_share_base}; fair_share_remainder: {fair_share_remainder}");
    NetOwed {
        fair_share: fair_share_base,
        total_amount,
        net_paid_in,
    }
}

pub fn greedy_min_cash_flow(paid_in: Vec<PaidIn>) -> PayOut {
    let converted: Vec<ConvertedPaidIn> = paid_in.iter().map(|x| ConvertedPaidIn{username: x.username.clone(), amount: Currency::new(x.amount)}).collect();
    let net_owed = calc_net_owed(converted);
    let mut debtors: Vec<(String, Currency)> = net_owed
        .net_paid_in
        .iter()
        .filter(|&(_, &net_amount)| net_amount < Currency::zero())
        .map(|(name, &net_amount)| (name.clone(), -net_amount))
        .collect();

    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut creditors: Vec<(String, Currency)> = net_owed
        .net_paid_in
        .iter()
        .filter(|&(_, &net_amount)| net_amount > Currency::zero())
        .map(|(name, &net_amount)| (name.clone(), net_amount))
        .collect();

    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

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

        debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        debtors.retain(|x| x.1 > Currency::zero());
        creditors.retain(|x| x.1 > Currency::zero());
    }
    info!("transactions: {transactions:?}");
    PayOut {
        fair_share: net_owed.fair_share.to_string(),
        total_amount: net_owed.total_amount.to_string(),
        transactions,
    }
}
