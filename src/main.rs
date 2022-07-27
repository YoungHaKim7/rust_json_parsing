#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let todos = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .text()
        .await?;
    println!("{todos:#?}");

    Ok(())
}
