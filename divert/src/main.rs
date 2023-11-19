use reqwest; // 异步 HTTP 客户端

#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = "BQDcaHkINbzehjbfx-odxpifboW64H2Dsd-NZSfTgfFeXitKy0TVHOlR_rKClMQPnWxAQ0v0gWQitWVeJejw_mzPwtQe6mz5buxIK2gLdM6EllVZUXGiM2HHRyL_6OtFQHfnpXlcU2--4tglRUsNhGnbdWbaqGNJErWrJ83nzqfrWDjYTSiIlVMZsKWqCzhHAXXb9qlBqTnah5phe0_EfrGp6g"; // 这里替换为你的访问令牌
    // 打印AccessToken
    println!("Using access token: {}", access_token);

    // 获取当前用户播放列表 https://api.spotify.com/v1/me
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

    // 获取用户关注的艺人 https://api.spotify.com/v1/me/following
    let cilent2 = reqwest::Client::new();
    let response2 = cilent2
        .get("https://api.spotify.com/v1/me/following")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if response2.status().is_success() {
        let response_body2 = response2.text().await?;
        println!("Response2: {}", response_body2);
        // 可以进一步处理 JSON 响应体，比如提取歌曲信息
    } else {
        // 处理错误情况，比如打印状态码
        println!("Failed to get the liked songs, status: {}", response2.status());
    }

    // 获取当前用户的播放列表 https://api.spotify.com/v1/me/playlists
    let playlist_cilent = reqwest::Client::new();
    let playlist_response = playlist_cilent
        .get("https://api.spotify.com/v1/me/playlists")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if playlist_response.status().is_success() {
        let playlist_response_body = playlist_response.text().await?;
        println!("当前用户播放列表是: {}", playlist_response_body);
        // 可以进一步处理 JSON 响应体，比如提取歌曲信息
    } else {
        // 处理错误情况，比如打印状态码
        println!("Failed to get the liked songs, status: {}", playlist_response.status());
    }

    // 获取当前用户的关注专辑 https://api.spotify.com/v1/me/albums
    let albums_cilent = reqwest::Client::new();
    let albums_response = albums_cilent
        .get("https://api.spotify.com/v1/me/albums")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if albums_response.status().is_success() {
        let albums_response_body = albums_response.text().await?;
        println!("当前用户关注的专辑是: {}", albums_response_body);
        // 可以进一步处理 JSON 响应体，比如提取歌曲信息
    } else {
        // 处理错误情况，比如打印状态码
        println!("Failed to get the liked songs, status: {}", albums_response.status());
    }

    // 获取当前用户的关注有声读物 https://api.spotify.com/v1/me/audiobooks
    let audiobooks_cilent = reqwest::Client::new();
    let audiobooks_response = audiobooks_cilent
        .get("https://api.spotify.com/v1/me/audiobooks")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if audiobooks_response.status().is_success() {
        let audiobooks_response_body = audiobooks_response.text().await?;
        println!("当前用户关注的有声读物是: {}", audiobooks_response_body);
        // 可以进一步处理 JSON 响应体，比如提取歌曲信息
    } else {
        // 处理错误情况，比如打印状态码
        println!("Failed to get the liked songs, status: {}", audiobooks_response.status());
    }
    Ok(())
}
