use reqwest; // 异步 HTTP 客户端
use serde_json; // JSON 的序列化和反序列化

#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "BQAem8xH-jUN5Nh9tNWfxKYokOdUPay_zdPY_Wg4cLo0HwyFu93niEVdp_fS4L0SYiIdGLiVroDIBjBKXe3M_PilLLfntr_6Sk2VhfXEbU8qqkQuTDuIpRykGXA7gzAedlmc-ShxzY7JHi5SIET_vliv7dazbv1lcERHnv34Qv9BuAIBRWw6OHsYk-OiuEOMyUjjvuwIxFuCE0Ux4LEQyUmv4A"; // 这里替换为你的访问令牌
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
