use scraper::Html;
use input_rust::input;

#[tokio::main]

async fn main() -> Result<(), reqwest::Error>{

    // Make and take input via input-rust crate
    let input = input("Please enter in a link: ").unwrap();
    // Make the web request
    let request = reqwest::get(input).await?;
    println!("Response: {:?} {}", request.version(), request.status());


    let body = request.text().await?;
    println!("{body}");

    Ok(())
}

