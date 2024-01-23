use actix_session::{CookieSession, Session};
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct AuthQuery {
    state: String, // 假设前端会传递 'source' 或 'target' 作为 state 参数
}

#[derive(Serialize, Deserialize)]
struct AuthTokenResponse {
    access_token: String,
    // Include any other fields as needed
}
#[derive(Deserialize)]
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
    uri: String, // The URI is needed to add the track to the playlist
                 // include other fields as necessary
}

#[derive(Deserialize)]
struct TracksList {
    tracks: Vec<String>,
}

#[derive(Deserialize)]
struct SpotifyPlaylistResponse {
    items: Vec<PlaylistItem>,
}

#[derive(Deserialize)]
struct PlaylistItem {
    name: String,
    tracks: PlaylistTracks,
    public: bool,
    collaborative: bool,
    owner: PlaylistOwner,
}

#[derive(Deserialize)]
struct PlaylistTracks {
    total: i32,
}

#[derive(Deserialize)]
struct PlaylistOwner {
    display_name: String,
}

// 登录被转移账号
async fn login_source(session: Session) -> impl Responder {
    dotenv().ok(); // 调用这个函数来读取 .env 文件
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

    // 打印
    println!("SPOTIFY_CLIENT_ID: {:?}", env::var("SPOTIFY_CLIENT_ID"));
    println!(
        "SPOTIFY_CLIENT_SECRET: {:?}",
        env::var("SPOTIFY_CLIENT_SECRET")
    );

    // 直接设置 "source" 作为 state 的值
    let state_value = "source";
    session.insert("login_state", state_value).unwrap();

    let scope = "user-library-read user-library-modify playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private user-follow-read user-follow-modify";
    let spotify_url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}&state={}&show_dialog=true",
        client_id, scope, redirect_uri, state_value
    );

    HttpResponse::Found()
        .append_header((header::LOCATION, spotify_url))
        .finish()
}



// 登录转移账号
async fn login_target(session: Session) -> impl Responder {
    dotenv().ok(); // 调用这个函数来读取 .env 文件
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

    println!("SPOTIFY_CLIENT_ID: {:?}", env::var("SPOTIFY_CLIENT_ID"));
    println!(
        "SPOTIFY_CLIENT_SECRET: {:?}",
        env::var("SPOTIFY_CLIENT_SECRET")
    );

    // 直接设置 "source" 作为 state 的值
    let state_value = "target";
    session.insert("login_state", state_value).unwrap();

    let scope = "user-library-read user-library-modify playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private user-follow-read user-follow-modify";
    let spotify_url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}&state={}&show_dialog=true",
        client_id, scope, redirect_uri, state_value
    );

    HttpResponse::Found()
        .append_header((header::LOCATION, spotify_url))
        .finish()
}

// // 登录被转移账号
// async fn login_source() -> impl Responder {
//     let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
//     let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

//     let scope = "user-read-private user-read-email playlist-modify-public playlist-read-private";
//     let spotify_url = format!(
//         "https://accounts.spotify.com/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&show_dialog=true",
//         client_id, redirect_uri, scope
//     );

//     HttpResponse::Found()
//         .append_header((header::LOCATION, spotify_url))
//         .finish()
// }

// // 登录转移账号
// async fn login_target() -> impl Responder {
//     let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
//     let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");

//     let scope = "user-read-private user-read-email playlist-modify-public playlist-read-private";
//     let spotify_url = format!(
//         "https://accounts.spotify.com/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&show_dialog=true",
//         client_id, redirect_uri, scope
//     );

//     HttpResponse::Found()
//         .append_header((header::LOCATION, spotify_url))
//         .finish()
// }

// 增加一个新的结构体来接收额外的查询参数
#[derive(Deserialize)]
struct SpotifyCallbackQuery {
    code: String,
    state: String, // 假设前端会传递 'source' 或 'target' 作为 state 参数
}
// 登录之后记录token并且返回主界面
async fn spotify_callback(
    info: web::Query<SpotifyCallbackQuery>,
    session: Session,
) -> impl Responder {
    dotenv().ok(); // 读取 .env 文件
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");
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
                // 根据 state 参数决定存储什么键
                let token_key = if info.state == "source" {
                    "access_token_source"
                } else {
                    "access_token_target"
                };
                // 尝试插入 token 到会话
                match session.insert(token_key, &auth_token_response.access_token) {
                    Ok(_) => HttpResponse::Ok().body(format!("{} stored in session", token_key)),
                    Err(_) => {
                        HttpResponse::InternalServerError().body("Failed to set session token")
                    }
                }
            }
            Err(_) => HttpResponse::BadRequest().body("Failed to parse response"),
        },
        Err(_) => HttpResponse::BadRequest().body("Failed to send request"),
    }
}

// // 获取关注的歌手
// async fn get_followed_artists(session: Session) -> impl Responder {
//     // 获取session
//     if let Ok(Some(access_token)) = session.get::<String>("access_token") {
//         // 创建HTTP客户端
//         let client = Client::new();
//         // 发送GET请求到Spotify API
//         let response = client
//             .get("https://api.spotify.com/v1/me/following?type=artist")
//             .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
//             .send()
//             .await;

//         // 处理响应
//         match response {
//             Ok(resp) => match resp.text().await {
//                 Ok(text) => HttpResponse::Ok()
//                     .content_type("application/json")
//                     .body(text),
//                 Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
//             },
//             Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),
//         }
//     } else {
//         HttpResponse::Unauthorized().body("No access_token found in session")
//     }
// }


// 获取关注的歌手，并考虑账号转移所需参数
async fn get_followed_artists(session: Session) -> impl Responder {
    // 从会话中获取source_access_token
    if let Ok(Some(access_token_source)) = session.get::<String>("access_token_source") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API获取关注的歌手
        let response = client
            .get("https://api.spotify.com/v1/me/following?type=artist")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token_source))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<serde_json::Value>().await {
                        Ok(artists_data) => {
                            // 解析歌手数据并提取需要的信息
                            let artists_info: Vec<_> = artists_data["artists"]["items"]
                                .as_array()
                                .unwrap_or(&vec![])
                                .iter()
                                .map(|artist| {
                                    // 除了前端所需的展示信息之外，还可以提取其他用于账号转移的数据
                                    serde_json::json!({
                                        "name": artist["name"].as_str().unwrap_or(""),
                                        "id": artist["id"].as_str().unwrap_or(""), // 保存ID用于转移
                                        // ...可以添加其他转移所需的信息
                                    })
                                })
                                .collect();

                            // 返回json响应
                            HttpResponse::Ok().json(artists_info)
                        }
                        Err(_) => HttpResponse::InternalServerError().body("Failed to parse artist data"),
                    }
                } else {
                    // 错误处理
                    HttpResponse::InternalServerError().body("Failed to get artists from Spotify")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to send request to Spotify"),
        }
    } else {
        HttpResponse::Unauthorized().body("No source access_token found in session")
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
            .get("https://api.spotify.com/v1/me/playlists")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<SpotifyPlaylistResponse>().await {
                        Ok(playlists) => {
                            // 创建一个新的 Vec，包含前端所需的歌单信息
                            let playlist_info: Vec<_> = playlists
                                .items
                                .into_iter()
                                .map(|item| {
                                    let PlaylistItem {
                                        name,
                                        tracks,
                                        public,
                                        collaborative,
                                        owner,
                                    } = item;
                                    serde_json::json!({
                                        "name": name,
                                        "tracks": tracks.total,
                                        "public": public,
                                        "collaborative": collaborative,
                                        "owner": owner.display_name,
                                    })
                                })
                                .collect();

                            // 返回json响应
                            HttpResponse::Ok().json(playlist_info)
                        }
                        Err(_) => {
                            HttpResponse::InternalServerError().body("Faild to parse playlist")
                        }
                    }
                } else {
                    // 打印出错误状态码和错误消息
                    let status_code = resp.status();
                    let error_message = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Failed to read error message".to_string());
                    println!("Error status: {}, Message: {}", status_code, error_message);
                    HttpResponse::InternalServerError().body(format!(
                        "Failed to get playlist from Spotify: {}",
                        error_message
                    ))
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request to Spotify"),
        }
        // match response {
        //     Ok(resp) => match resp.text().await {
        //         Ok(text) => HttpResponse::Ok()
        //             .content_type("application/json")
        //             .body(text),
        //         Err(_) => HttpResponse::InternalServerError().body("Faild to read response body"),
        //     },
        //     Err(_) => HttpResponse::InternalServerError().body("Faild to send request"),
        // }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}

// 歌单列表添加到Spotify歌单
async fn search_and_add_tracks(
    tracks_list: web::Json<TracksList>,
    session: Session,
) -> impl Responder {
    // 从会话中获取access_token
    if let Ok(Some(access_token)) = session.get::<String>("access_token") {
        // 创建htpp客户端
        let client = Client::new();

        let mut track_uris = Vec::new();
        for track_name in &tracks_list.tracks {
            // Spotify API 搜索歌曲
            let search_response = client
                .get(format!(
                    "https://api.spotify.com/v1/search?q={}&type=track",
                    track_name
                ))
                .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
                .send()
                .await;

            if let Ok(resp) = search_response {
                if let Ok(search_result) = resp.json::<SearchResponse>().await {
                    // 提取歌曲URI
                    if let Some(track) = search_result.tracks.items.first() {
                        track_uris.push(track.uri.clone());
                    }
                }
            }
        }

        // 将歌曲添加到用户的Spotify歌单
        let playlist_response = client
            .post("https://api.spotify.com/v1/playlists/{playlist_id}/tracks") // 替换 {playlist_id} 为实际的歌单ID
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            .json(&serde_json::json!({ "uris": track_uris }))
            .send()
            .await;

        match playlist_response {
            Ok(resp) => HttpResponse::Ok()
                .content_type("application/json")
                .body(resp.text().await.unwrap()),
            Err(_) => {
                HttpResponse::InternalServerError().body("Failed to add tracks to the playlist")
            }
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
            .route("/login/source", web::get().to(login_source))
            .route("/login/target", web::get().to(login_target))
            .route("/callback", web::get().to(spotify_callback))
            .route("/artist", web::get().to(get_followed_artists))
            .route("/tracks", web::get().to(get_followed_tracks))
            .route("/playlist", web::get().to(get_followed_playlist))
            .route("/addTrackToPlaylist", web::get().to(search_and_add_tracks))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
