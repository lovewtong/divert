use reqwest;
use base64;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client_id = "your_client_id";
    let client_secret = "your_client_secret";
    let redirect_uri = "your_redirect_uri";
    let code = "your_code"; // 授权回调中获取的 code
    let scopes = "user-library-read"; // 请求的权限

    let auth_value = base64::encode(format!("{}:{}", client_id, client_secret));

    let client = reqwest::Client::new();

    let mut data = HashMap::new();
    data.insert("redirect_uri", redirect_uri);
    data.insert("code", code);
    data.insert("grant_type", "authorization_code");
    data.insert("scope", scopes);

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", auth_value))
        .form(&data)
        .send()
        .await?;

    let token_info: Value = response.json().await?;

    println!("Token info: {:?}", token_info);
    Ok(())
}
