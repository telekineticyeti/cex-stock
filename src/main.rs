extern crate reqwest;
extern crate serde_json;
#[macro_use] extern crate error_chain;

use serde::Deserialize;
use reqwest::Response;

mod errors {
    error_chain! {
        foreign_links {
            ReqError(reqwest::Error);
            JsonError(serde_json::Error);
        }
    }
}

use errors::*;

#[derive(Deserialize, Debug)]
struct ResDaddy {
    response: Body,
}

#[derive(Deserialize, Debug)]
struct Body {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    boxes: Vec<Product>,
    #[serde(rename="totalRecords")]
    total_records: u32,
    #[serde(rename="minPrice")]
    min_price:  u32,
    #[serde(rename="maxPrice")]
    max_price: u32,
}

#[derive(Deserialize, Debug)]
struct Product {
    #[serde(rename="boxId")]
    id: String,
    #[serde(rename="boxName")]
    name: String,
    #[serde(rename="outOfStock")]
    out_of_stock: u32,
    #[serde(rename="outOfEcomStock")]
    out_of_eshop_stock: u32,
    #[serde(rename="sellPrice")]
    price: u32,
}

fn main() {
    let search: &str = "audioengine";
    let query = make_search_url(search);
    println!("{:#?}", get_raw_search(&query));
}

fn make_search_url(search_term: &str) -> String {
    let base_url = "https://wss2.cex.uk.webuy.io/v3/boxes?q=";
    let url_string_options = "&firstRecord=1&count=5&sortBy=relevance&sortOrder=desc";
    let url: String = format!("{}{}{}", base_url, search_term, url_string_options);

    url
}

fn get_raw_search(url: &str) -> Result<()> {
    let mut response: Response = reqwest::get(url)?;
    let test: ResDaddy = response.json()?;

    println!("Products: {}", test.response.data.boxes.len());

    for product in test.response.data.boxes.iter() {
        println!("{:?}", product.name);
    }

    Ok(())
}
