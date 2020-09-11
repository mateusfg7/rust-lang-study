
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://0.0.0.0:4507/")
        .await?
        .text()
        .await?;
    println!("{}", resp);
    Ok(())
}