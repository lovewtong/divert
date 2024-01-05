use actix_session::{CookieSession, Session};
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct AuthQuery {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct AuthTokenResponse {
    access_token: String,
    // Include any other fields as needed
}

struct SearchResponse {
    tracks: Tracks,
}

#[derive(Deserialize)]
struct Tracks {
    items: Vec<TrackItem>,
}

#[derive(Deserialize)]
struct TrackItem {
    id: String,
    // include other fields as necessary
}

// 登录
async fn spotify_login() -> impl Responder {

    dotenv().ok(); // 调用这个函数来读取 .env 文件
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

    println!("SPOTIFY_CLIENT_ID: {:?}", env::var("SPOTIFY_CLIENT_ID"));
    println!("SPOTIFY_CLIENT_SECRET: {:?}", env::var("SPOTIFY_CLIENT_SECRET"));

    let scope = "user-library-read user-library-modify playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private user-follow-read user-follow-modify";
    let spotify_url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}",
        client_id, scope, redirect_uri
    );
    HttpResponse::Found()
        .append_header((header::LOCATION, spotify_url))
        .finish()
}

// 登录之后记录token并且返回
async fn spotify_callback(info: web::Query<AuthQuery>, session: Session) -> impl Responder {

    dotenv().ok(); // 调用这个函数来读取 .env 文件
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
    
    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &info.code),
        ("redirect_uri", &redirect_uri),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<AuthTokenResponse>().await {
            Ok(auth_token_response) => {
                session
                    .insert("access_token", &auth_token_response.access_token)
                    .unwrap();
                HttpResponse::Ok().body("access_token stored in session")
            }
            Err(_) => HttpResponse::BadRequest().body("Failed to parse response"),
        },
        Err(_) => HttpResponse::BadRequest().body("Failed to send request"),
    }
}

// 获取关注的歌手
async fn get_followed_artists(session: Session) -> impl Responder {
    // 获取session
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client
            .get("https://api.spotify.com/v1/me/following?type=artist")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => match resp.text().await {
                Ok(text) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(text),
                Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),
        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}

// 获取关注的歌曲
async fn get_followed_tracks(session: Session) -> impl Responder {
    // 获取session
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client
            .get("https://api.spotify.com/v1/me/tracks")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => match resp.text().await {
                Ok(text) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(text),
                Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),
        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}

// 获取关注的歌单
async fn get_followed_playlist(session: Session) -> impl Responder {
    // 获取session
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client
            .get("https://api.spotify.com/v1/me/playlist")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => match resp.text().await {
                Ok(text) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(text),
                Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),
        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}

async fn search_and_add_tracks(track_names: Vec<String>, access_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Search for each track and collect their Spotify IDs
    let mut track_ids = Vec::new();
    for track_name in track_names {
        let search_response = client
            .get(format!("https://api.spotify.com/v1/search?q={}&type=track", track_name))
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<SearchResponse>() // Define SearchResponse according to the structure of the response
            .await?;

        if let Some(track_id) = search_response.tracks.items.first().map(|item| &item.id) {
            track_ids.push(track_id.clone());
        }
    }

    // Add tracks to the playlist
    let playlist_id = "your_playlist_id"; // Replace with the actual playlist ID
    client
        .post(format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .json(&serde_json::json!({ "uris": track_ids }))
        .send()
        .await?;

    Ok(())
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    HttpServer::new(|| {
        App::new()
            // 配置基于 Cookie 的会话中间件，注意在生产环境中使用安全的密钥
            .wrap(CookieSession::signed(&[0; 32]).secure(false)) // 在生产环境中应该使用 `.secure(true)` 和 HTTPS
            .route("/login", web::get().to(spotify_login))
            .route("/callback", web::get().to(spotify_callback))
            .route("/artist", web::get().to(get_followed_artists))
            .route("/tracks", web::get().to(get_followed_tracks))
            .route("/playlist", web::get().to(get_followed_playlist))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
