use std::{
    collections::HashMap,
    io::{self, Write},
};


use axum::{routing::post, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

struct PaidIn {
    username: String,
    amount: f32
}

fn calc_net_owed(paid_in: Vec<PaidIn>) -> HashMap<String, f32> {
    let total_amount: f32 = paid_in.iter().fold(0.0, |acc, x| acc + x.amount);
    let fair_share = total_amount / paid_in.len() as f32;
    println!("fair_share: {fair_share}");
    let net_paid_in: HashMap<String, f32> = paid_in
        .iter()
        .map(|x| (x.username.clone(), x.amount - fair_share))
        .collect();
    println!("{net_paid_in:?}");
    net_paid_in
}

fn greedy_min_cash_flow(net_owed: HashMap<String, f32>) -> Vec<String>{
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

    let mut result: Vec<String> = Vec::new();

    while debtors.len() > 0 && creditors.len() > 0 {
        let biggest_creditor = creditors.get(0).expect("none found");
        let biggest_debtor = debtors.get(0).expect("none found");

        let debtor_pays = biggest_creditor.1.min(biggest_debtor.1);
        let biggest_debtor = biggest_debtor.0.clone();
        let biggest_creditor = biggest_creditor.0.clone();
        result.push(format!("{biggest_debtor} pays {biggest_creditor} {debtor_pays}"));
        println!("{biggest_debtor} pays {biggest_creditor} {debtor_pays}");

        creditors[0].1 -= debtor_pays;
        debtors[0].1 -= debtor_pays;

        debtors.retain(|x| x.1 > 0.01);
        creditors.retain(|x| x.1 > 0.01);

        debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }
    result
}


async fn hello_world() -> &'static str {
    "root"
}

async fn create_payout(Json(payload): Json<HashMap<String, f32>>) -> Json<Vec<String>> {
    let payouts: Vec<PaidIn> = payload
            .into_iter()
            .map(|(username, amount)| PaidIn { username, amount })
            .collect();
    let owed = calc_net_owed(payouts);
    Json(greedy_min_cash_flow(owed))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/users", post(create_payout));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
