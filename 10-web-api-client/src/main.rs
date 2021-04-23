use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("request_url: {}", request_url);

    // MUST specify user agent for this request to work!
    let client = reqwest::Client::builder()
        .user_agent("APP_USER_AGENT")
        .build()?;

    let response = client.get(&request_url).send().await?;
    println!("response: {:?}", response);

    let users: Vec<User> = response.json().await?;
    for user in &users {
        println!("{:?}", user);
    }
    
    Ok(())
}
