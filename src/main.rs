#![allow(unused_imports)]
use rand::Rng;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize}; // 이 줄을 추가하세요
use king::*;
use rayon::prelude::*;
mod sum;

fn main(){
    println!("Hello, world!");
    template();



}

fn template(){
    println!("Hello, world!");
    let a=sum::sum(1,2);
    println!("{}",a);

    let mut n = 0;
    let mut people: Vec<Value> = vec![];
    let mut rng = rand::thread_rng();
    while n < 100 {
        let name = format!("name{}", rng.gen_range(1..=9999));
        let age = rng.gen_range(1..=70);
        let person = json!({
            "name": name,
            "age": age,
        });
        people.push(person);
        n = n + 1;
    }
    let people_json = serde_json::to_string(&people).unwrap();
    println!("{}", people_json);
    let decoded_people: Vec<Value> = serde_json::from_str(&people_json).unwrap();
    println!("{:?}", decoded_people);
    let odd_age = &decoded_people[0]["age"].as_i64().unwrap() + 99999;
    println!("{}", odd_age);
    let select_persion = &decoded_people[0];
    let print_name = format!("{}qqqqq", select_persion["name"].as_str().unwrap());
    println!("{}", print_name);


    // Binance API
    let binance_data = binance().unwrap();
    let binance_data_json: Vec<Value> = serde_json::from_str(&binance_data).unwrap();
    println!("\n{:?}", binance_data_json);
    //let binance_data_price = binance_data_json[0][1].as_f64().unwrap();
    println!("\n{}", binance_data_json[0][1].as_str().unwrap().parse::<f64>().unwrap()+999999.99);

    // Binance Ticker API
    let binance_ticker_data = ticker().unwrap();
    let binance_ticker_data_json: Vec<StructTicker> = serde_json::from_str(&binance_ticker_data).unwrap();
    println!("\n{:?}", binance_ticker_data_json[0]);
    //symbol이 BTCUSDT인 price를 가져옵니다.
    let mut i = 0;
    while i < binance_ticker_data_json.len() {
        if binance_ticker_data_json[i].symbol == "BTCUSDT" {
            println!("{}", binance_ticker_data_json[i].price);
            break;
        }
        i = i + 1;
    }    
}
#[tokio::main]
async fn ticker() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let request = client.request(reqwest::Method::GET, "https://api.binance.com/api/v3/ticker/price");

    let response = request.send().await?;
    let body = response.text().await?;

    //println!("{}", body);

    Ok(body)
}

#[tokio::main]
async fn binance() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let request = client.request(reqwest::Method::GET, "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1m&limit=3");

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(body)
}

#[derive(Debug, Deserialize, Serialize)]
struct StructTicker {
    symbol: String,
    price: String,
}
