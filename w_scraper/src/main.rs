use rusqlite::Connection;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct ProductInfo {
    image: String,
    name: String,
    price: String,
}

fn get_urls(link: &str) -> Result<Vec<String>, reqwest::Error> {
    let html = reqwest::blocking::get(link)?
    .text()?;

    let document = Html::parse_document(&html);

    let link_selector = Selector::parse("a[href]").unwrap();

    let mut result = vec![];
    for element in document.select(&link_selector) {
        if let Some(link) = element.value().attr("href") {
            if !(link.starts_with('#') || link.starts_with('?')) {
                result.push(link.to_string());
            }

        }
    }
    Ok(result)
}

fn get_product_info(link: &str) -> Result<ProductInfo, reqwest::Error> {
    let html = reqwest::blocking::get(link)?
        .text()?;
    let document = Html::parse_document(&html);

    let mut p_info = ProductInfo {
        image: String::new(),
        name: String::new(),
        price: String::new(),
    };

    let image_selector = Selector::parse("div[data-thumb]").unwrap();
    if let Some(element) = document.select(&image_selector).next() {
        if let Some(thumb) = element.value().attr("data-thumb") {
            p_info.image = thumb.to_string();
        }
    }

    let price_selector = Selector::parse(".woocommerce-Price-amount bdi").unwrap();
    if let Some(element) = document.select(&price_selector).next() {
        p_info.price = element.text().collect();
    }

    let title_selector = Selector::parse("title").unwrap();
    if let Some(title_element) = document.select(&title_selector).next() {
        let full_title: String = title_element.text().collect();
        if let Some(product_name) = full_title.split('–').next() {
            p_info.name = product_name.trim().to_string();
        }
    }

    Ok(p_info)
}

fn init_db(path: &str) -> Connection {
    let conn = Connection::open(path).expect("could not open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT,
            price TEXT,
            image TEXT
        )",
        [],
    ).expect("could not create table");
    conn
}

fn insert_product(conn: &Connection, product: &ProductInfo) {
    conn.execute(
        "INSERT INTO products (name, price, image) VALUES (?1, ?2, ?3)",
        [&product.name, &product.price, &product.image],
    ).expect("could not insert product");
}

fn main() -> Result<(), reqwest::Error> {
    let conn = init_db("products.db");
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    queue.push_back("https://www.scrapingcourse.com/ecommerce/".to_string());

    while let Some(url) = queue.pop_front() {
        if visited.contains(&url) {
            continue;
        }
        visited.insert(url.clone());
        println!("{}", url);
        let found_links = get_urls(&url)?;

        if url.contains("/product/") {
            let product_info = get_product_info(&url)?;
            println!("{:?}", product_info);
            if !product_info.image.is_empty() {
                insert_product(&conn, &product_info);
            }
        }

        for link in found_links {
            if !visited.contains(&link) && link.contains("www.scrapingcourse.com") {
                queue.push_back(link.clone());
            }

        }
    }

    Ok(())
}
