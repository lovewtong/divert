use actix_session::{CookieSession, Session};
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AuthQuery {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct AuthTokenResponse {
    access_token: String,
    // Include any other fields as needed
}

async fn spotify_login() -> impl Responder {
    let client_id = "71b3175264c3491ea0a0103e72b469f2";
    let redirect_uri = "http://localhost:8080/callback";
    let scope = "user-library-read user-library-modify playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private user-follow-read user-follow-modify";
    let spotify_url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}",
        client_id, scope, redirect_uri
    );
    HttpResponse::Found()
        .append_header((header::LOCATION, spotify_url))
        .finish()
}

async fn spotify_callback(info: web::Query<AuthQuery>, session: Session) -> impl Responder {
    let client_id = "71b3175264c3491ea0a0103e72b469f2";
    let client_secret = "522e3004ba824a70898c96c9496ead7e";
    let redirect_uri = "http://localhost:8080/callback";

    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &info.code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<AuthTokenResponse>().await {
            Ok(auth_token_response) => {
                session.insert("access_token", &auth_token_response.access_token).unwrap();
                HttpResponse::Ok().body("access_token stored in session")
            }
            Err(_) => HttpResponse::BadRequest().body("Failed to parse response"),
        },
        Err(_) => HttpResponse::BadRequest().body("Failed to send request"),
    }
}

// 获取关注的歌手
async fn get_followed_artists(session: Session) -> impl Responder{

    // 获取session
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {

        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client.get("https://api.spotify.com/v1/me/following?type=artist")
            .header(header::AUTHORIZATION, format!("Bearer {}",access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => match resp.text().await{
                Ok(text) => HttpResponse::Ok().content_type("application/json").body(text),
                Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),

        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}

// 获取关注的歌曲
async fn get_followed_tracks(session: Session) -> impl Responder{

    // 获取session
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {

        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client.get("https://api.spotify.com/v1/me/tracks")
            .header(header::AUTHORIZATION, format!("Bearer {}",access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => match resp.text().await{
                Ok(text) => HttpResponse::Ok().content_type("application/json").body(text),
                Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),

        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}

// 获取关注的歌单
async fn get_followed_playlist(session: Session) -> impl Responder{

    // 获取session
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {

        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client.get("https://api.spotify.com/v1/me/playlist")
            .header(header::AUTHORIZATION, format!("Bearer {}",access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => match resp.text().await{
                Ok(text) => HttpResponse::Ok().content_type("application/json").body(text),
                Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),

        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
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
