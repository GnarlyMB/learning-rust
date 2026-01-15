use scraper::Html;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use input_rust::input;

#[tokio::main]

async fn main() {

    // Make and take input via input-rust crate
    let input = input("Please enter in a link: ").unwrap();
    // Make the web request
    let body = reqwest::get(input)
        .await
        .text()
        .await;
    println!("body = {body:?}");

}

