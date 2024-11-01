// 사용하지 않는 import로 인한 컴파일 경고를 무시합니다.
#![allow(unused_imports)]

// 필요한 크레이트와 모듈을 가져옵니다.
use colored::*;
use indicatif::{ProgressBar, ProgressStyle}; // 진행률 바를 표시하기 위한 라이브러리입니다.
use king::*; // 사용자 정의 모듈로, 다른 곳에서 정의되었을 것으로 가정합니다.
use rand::Rng; // 랜덤 숫자를 생성하기 위한 라이브러리입니다.
use rayon::prelude::*; // 병렬 처리를 위한 라이브러리입니다.
use serde::{Deserialize, Serialize}; // 데이터의 직렬화와 역직렬화를 위한 라이브러리입니다.
use serde_json::{json, Value}; // JSON 데이터를 다루기 위한 라이브러리입니다.
mod template;
// 프로그램의 진입점인 메인 함수입니다.

fn main() {
    // 콘솔에 인사말을 출력합니다.
    println!("Hello, world!");
    // 'template' 함수를 호출합니다.
    template::template();
}
