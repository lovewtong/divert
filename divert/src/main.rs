use reqwest; // 异步 HTTP 客户端
use serde_json; // JSON 的序列化和反序列化

#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "BQBTMB6j0nAIUBnla37sixgsvz7FOeQ1npZHgP4A6ZbhdEm6ICISzm74Nk6N5yUOXaKLG7jn0b047JMaonMwPXX3QDfRYVtkM175Id2LfOtAf94Dx_o"; // 这里替换为你的访问令牌
    println!("Using access token: {}", access_token);

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/following")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if response.status().is_success() {
        let response_body = response.text().await?;
        println!("Response: {}", response_body);
        // 可以进一步处理 JSON 响应体，比如提取歌曲信息
    } else {
        // 处理错误情况，比如打印状态码
        println!("Failed to get the liked songs, status: {}", response.status());
    }

    Ok(())
}
