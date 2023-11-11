use reqwest; // 异步 HTTP 客户端
use serde_json; // JSON 的序列化和反序列化

#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "BQBbD4k9azLL6pwU9tkJXd3oYb5otBubvo842H0-Qhghj9aDTcOLcphXd4aVO_rhIdmADg4xTfByRYUykBAmPv5GcFxHyp8CDrKczsWnMDD3WwR21EYx7x3DXBhZOfBJNvsSsGdU0sk2g5C_b_4ywfyaKjoSqOGtWL-riGoCg_mgXmNnIfxCUy3-Q3S0u7gETnf2JEJAJPZYdz4w3xfFzE4CUQ"; // 这里替换为你的访问令牌
    println!("Using access token: {}", access_token);

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me")
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
