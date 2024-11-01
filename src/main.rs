// 사용하지 않는 import로 인한 컴파일 경고를 무시합니다.
#![allow(unused_imports)]

// 필요한 크레이트와 모듈을 가져옵니다.
use indicatif::{ProgressBar, ProgressStyle}; // 진행률 바를 표시하기 위한 라이브러리입니다.
use king::*; // 사용자 정의 모듈로, 다른 곳에서 정의되었을 것으로 가정합니다.
use rand::Rng; // 랜덤 숫자를 생성하기 위한 라이브러리입니다.
use rayon::prelude::*; // 병렬 처리를 위한 라이브러리입니다.
use serde::{Deserialize, Serialize}; // 데이터의 직렬화와 역직렬화를 위한 라이브러리입니다.
use serde_json::{json, Value}; // JSON 데이터를 다루기 위한 라이브러리입니다.
use colored::*;


// 'sum.rs' 파일에서 'sum' 모듈을 가져옵니다.
mod sum;

// 프로그램의 진입점인 메인 함수입니다.
fn main() {
    // 콘솔에 인사말을 출력합니다.
    println!("Hello, world!");
    // 'template' 함수를 호출합니다.

    template();
    println!("GLOBAL_ABC {}", unsafe{GLOBAL_ABC});
    println!("{}", "This text is red".red());
}

// 전역 변수를 정의합니다.
static mut GLOBAL_ABC: i64 = 0;
// 대부분의 로직이 담긴 'template' 함수를 정의합니다.
fn template() {
    // 'sum' 모듈의 'sum' 함수를 사용하여 두 숫자를 더합니다.
    let a = sum::sum(1, 2);
    println!("{}", a);

    unsafe{
        GLOBAL_ABC = 123;
    }
    println!("GLOBAL_ABC {}", unsafe{GLOBAL_ABC});        
    // JSON 데이터를 생성하고 다루는 예제입니다.
    let mut n = 0;
    let mut people: Vec<Value> = vec![]; // 사람들을 저장할 벡터를 생성합니다.
    let mut rng = rand::thread_rng(); // 랜덤 숫자 생성기를 초기화합니다.
    while n < 100 {
        // 랜덤한 이름과 나이를 생성합니다.
        let name = format!("name{}", rng.gen_range(1..=9999));
        let age = rng.gen_range(1..=70);
        // 사람 객체를 JSON 형태로 생성합니다.
        let person = json!({
            "name": name,
            "age": age,
        });
        // 벡터에 사람을 추가합니다.
        people.push(person);
        n = n + 1;
    }
    // 사람들의 정보를 JSON 문자열로 변환합니다.
    let people_json = serde_json::to_string(&people).unwrap();
    println!("{}", people_json);
    // JSON 문자열을 다시 Vec<Value> 타입으로 역직렬화합니다.
    let decoded_people: Vec<Value> = serde_json::from_str(&people_json).unwrap();
    println!("{:?}", decoded_people);
    // 첫 번째 사람의 나이에 99999를 더합니다.
    let odd_age = &decoded_people[0]["age"].as_i64().unwrap() + 99999;
    println!("{}", odd_age);
    // 첫 번째 사람의 이름을 가져와서 문자열을 추가합니다.
    let select_persion = &decoded_people[0];
    let print_name = format!("{}qqqqq", select_persion["name"].as_str().unwrap());
    println!("{}", print_name);

    // 바이낸스 API를 호출하여 데이터 가져오기
    let binance_data = binance().unwrap();
    // JSON 문자열을 Vec<Value> 타입으로 변환합니다.
    let binance_data_json: Vec<Value> = serde_json::from_str(&binance_data).unwrap();
    println!("\n{:?}", binance_data_json);
    // 가격 데이터를 가져와서 999999.99를 더합니다.
    println!(
        "\n{}",
        binance_data_json[0][1]
            .as_str()
            .unwrap()
            .parse::<f64>()
            .unwrap()
            + 999999.99
    );

    // 바이낸스 티커 API 호출
    let binance_ticker_data = ticker().unwrap();
    // JSON 문자열을 StructTicker 구조체의 벡터로 변환합니다.
    let binance_ticker_data_json: Vec<StructTicker> =
        serde_json::from_str(&binance_ticker_data).unwrap();
    println!("\n{:?}", binance_ticker_data_json[0]);
    // symbol이 'BTCUSDT'인 가격을 가져옵니다.
    let mut i = 0;
    while i < binance_ticker_data_json.len() {
        if binance_ticker_data_json[i].symbol == "BTCUSDT" {
            println!("{}", binance_ticker_data_json[i].price);
            break;
        }
        i = i + 1;
    }

    // 병렬 처리를 사용하여 원본 데이터를 변경하지 않고 새로운 데이터를 생성합니다.
    let original: Vec<i32> = vec![0; 100]; // 0으로 채워진 길이 100의 벡터를 생성합니다.
    let made = original
        .par_iter()
        .enumerate()
        .map(|(n, x)| if n > 10 { x + n as i32 } else { 0 })
        .collect::<Vec<_>>();
    println!("{:?}", original);
    println!("{:?}", made);

    // 병렬 처리를 사용하여 필요한 데이터를 바로 생성합니다.
    let made: Vec<i32> = (0..100)
        .into_par_iter()
        .enumerate()
        .map(|(n, x)| if n > 10 { x + n as i32 } else { 0 })
        .collect();
    println!("{:?}", made);

    // 진행률 바를 사용하여 작업의 진행 상황을 표시합니다.
    let pb = ProgressBar::new((100_000_000) as u64); // 총 작업량을 설정합니다.
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:30.cyan/blue}] 
            {human_pos}/{human_len} ({percent}%) ({eta}) {per_sec} {msg}",
            )
            .expect("템플릿을 설정할 수 없습니다.")
            .progress_chars("#>-"),
    );
    pb.set_message("...");

    let mut n = 0;
    while n < 100_000_000 {
        // 진행률 바를 업데이트합니다.
        if n % 10 == 0 {
            pb.inc(10);
        }
        n = n + 1;
    }
    // 작업 완료 메시지를 표시합니다.
    pb.finish_with_message("done");
}

// 비동기 함수로 바이낸스 티커 API를 호출하여 데이터를 가져옵니다.
#[tokio::main]
async fn ticker() -> Result<String, Box<dyn std::error::Error>> {
    // HTTP 클라이언트를 생성합니다.
    let client = reqwest::Client::builder().build()?;

    // GET 요청을 생성합니다.
    let request = client.request(
        reqwest::Method::GET,
        "https://api.binance.com/api/v3/ticker/price",
    );

    // 요청을 보내고 응답을 받습니다.
    let response = request.send().await?;
    // 응답 본문을 텍스트로 변환합니다.
    let body = response.text().await?;

    // 데이터를 반환합니다.
    Ok(body)
}

// 비동기 함수로 바이낸스 API를 호출하여 데이터를 가져옵니다.
#[tokio::main]
async fn binance() -> Result<String, Box<dyn std::error::Error>> {
    // HTTP 클라이언트를 생성합니다.
    let client = reqwest::Client::builder().build()?;

    // GET 요청을 생성합니다.
    let request = client.request(
        reqwest::Method::GET,
        "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1m&limit=3",
    );

    // 요청을 보내고 응답을 받습니다.
    let response = request.send().await?;
    // 응답 본문을 텍스트로 변환합니다.
    let body = response.text().await?;

    // 응답 본문을 출력합니다.
    println!("{}", body);

    // 데이터를 반환합니다.
    Ok(body)
}

// 티커 데이터를 저장하기 위한 구조체를 정의합니다.
#[derive(Debug, Deserialize, Serialize)]
struct StructTicker {
    symbol: String, // 심볼 이름 (예: "BTCUSDT")
    price: String,  // 해당 심볼의 가격
}
