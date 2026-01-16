use scraper::{Html, Selector};
use input_rust::input;

#[tokio::main]

async fn main() -> Result<(), reqwest::Error>{

    // Make and take input via input-rust crate
    let input = input("Please enter in a link: ").unwrap();
    // Make the web request
    let request = reqwest::get(input).await?;
    println!("Response: {:?} {}", request.version(), request.status());


    let body = request.text().await?;
    // println!("{body}");
    let html_pull = Html::parse_document(&body);
    // Grab the src and href
    let src = Selector::parse("src");
    let href = Selector::parse("href");


    //println!("{:#?}", html_pull);

    Ok(())
}

