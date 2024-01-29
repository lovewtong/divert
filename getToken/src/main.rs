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

// spotify_callback结构体:增加一个新的结构体来接收额外的查询参数
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

#[derive(Deserialize)]
struct SpotifyAlbumsResponse {
    items: Vec<AlbumItem>,
}

#[derive(Deserialize)]
struct AlbumItem {
    album: Album,
}

#[derive(Deserialize)]
struct Album {
    id: String,
    name: String,
    artists: Vec<Artist>,
    total_tracks: u32, // 歌曲数量
}
// 获取保存的专辑
async fn get_saved_albums(session: Session) -> impl Responder {
    if let Ok(Some(access_token_source)) = session.get::<String>("access_token_source") {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.spotify.com/v1/me/albums")
            .header(header::AUTHORIZATION, format!("Bearer {}", access_token_source))
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<SpotifyAlbumsResponse>().await {
                        Ok(albums_response) => {
                            let album_info: Vec<_> = albums_response.items.into_iter().map(|item| {
                                serde_json::json!({
                                    "id": item.album.id,
                                    "title": item.album.name,
                                    "artist": item.album.artists.iter().map(|artist| artist.name.clone()).collect::<Vec<String>>(),
                                    "tracks": item.album.total_tracks,
                                })
                            }).collect();

                            HttpResponse::Ok().json(album_info)
                        }
                        Err(_) => HttpResponse::InternalServerError().body("Failed to parse albums"),
                    }
                } else {
                    let status_code = resp.status();
                    let error_message = resp.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
                    println!("Error status: {}, MESSAGE: {}", status_code, error_message);
                    HttpResponse::InternalServerError().body(format!(
                        "Failed to get albums from Spotify: {}",
                        error_message
                    ))
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to send request to Spotify"),
        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
}




#[derive(Deserialize)]
struct SpotifyArtistResponse {
    artists: ArtistsResponse
}
#[derive(Deserialize)]
struct ArtistsResponse {
    items: Vec<ArtistItem>,
}
#[derive(Deserialize)]
struct ArtistItem {
    id: String,
    name: String,
}
// 获取关注的歌手
async fn get_followed_artists(session: Session) -> impl Responder {
    // 从会话中获取source_access_token
    if let Ok(Some(access_token_source)) = session.get::<String>("access_token_source") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API获取关注的歌手
        let response = client
            .get("https://api.spotify.com/v1/me/following?type=artist")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", access_token_source),
            )
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<SpotifyArtistResponse>().await {
                        Ok(artist) => {
                            // 创建一个新的Vec，包含前端所需的歌曲信息
                            let artist_info: Vec<_> = artist
                                .artists.items
                                .into_iter()
                                .map(|item| {
                                    serde_json::json!({
                                        "id": item.id,
                                        "name": item.name,
                                    })
                                })
                                .collect();

                            // 返回json反向
                            HttpResponse::Ok().json(artist_info)
                        }
                        Err(_) => HttpResponse::InternalServerError().body("Faild to parse artist"),
                    }
                } else {
                    // 打印错误状态码和错误消息
                    let status_code = resp.status();
                    let error_messgae = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Failed to read error message".to_string());
                    println!("Error status: {}, MESSAGE: {}", status_code, error_messgae);
                    HttpResponse::InternalServerError().body(format!(
                        "Failed to get artist from Spotify: {}",
                        error_messgae
                    ))
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request to Spotify"),
        }
    } else {
        HttpResponse::Unauthorized().body("No source access_token found in session")
    }
}

#[derive(Deserialize)]
struct SpotifyTracksResponse {
    items: Vec<TracksItem>,
}
#[derive(Deserialize)]
struct TracksItem {
    // added_at: String, //添加时间
    track: Track,
}
#[derive(Deserialize)]
struct Track {
    album: Album,
    artists: Vec<Artist>,
    // ... 其他字段
    name: String,
    // ...
    id: String,
}
#[derive(Deserialize)]
struct Artist {
    // ... 相关字段
    name: String,
    // ...
}
// 获取关注的歌曲
async fn get_followed_tracks(session: Session) -> impl Responder {
    // 获取access_session_source
    if let Ok(Some(access_token_source)) = session.get::<String>("access_token_source") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client
            .get("https://api.spotify.com/v1/me/tracks")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", access_token_source),
            )
            .send()
            .await;

        // 处理响应
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<SpotifyTracksResponse>().await {
                        Ok(tracks_response) => {
                            // 创建一个新的Vec，包含前端所需的歌曲信息
                            let tracks_info: Vec<_> = tracks_response
                                .items
                                .into_iter()
                                .map(|item| {
                                    serde_json::json!({
                                        "id": item.track.id,
                                        "title": item.track.name,
                                        "artist": item.track.album.name,
                                        "album": item.track.artists.iter().map(|artist| &artist.name).collect::<Vec<&String>>()
                                    })
                                })
                                .collect();

                            // 返回json反向
                            HttpResponse::Ok().json(tracks_info)
                        }
                        Err(_) => HttpResponse::InternalServerError().body("Faild to parse tracks"),
                    }
                } else {
                    // 打印错误状态码和错误消息
                    let status_code = resp.status();
                    let error_messgae = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Failed to read error message".to_string());
                    println!("Error status: {}, MESSAGE: {}", status_code, error_messgae);
                    HttpResponse::InternalServerError().body(format!(
                        "Failed to get playlist from Spotify: {}",
                        error_messgae
                    ))
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Faild to send request to Spotify"),
        }
    } else {
        HttpResponse::Unauthorized().body("No access_token found in session")
    }
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
    id: String,
}
// 获取关注的歌单
async fn get_followed_playlist(session: Session) -> impl Responder {
    // 获取session
    if let Ok(Some(access_token_source)) = session.get::<String>("access_token_source") {
        // 创建HTTP客户端
        let client = Client::new();
        // 发送GET请求到Spotify API
        let response = client
            .get("https://api.spotify.com/v1/me/playlists")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", access_token_source),
            )
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
                                        id,
                                    } = item;
                                    serde_json::json!({
                                        "name": name,
                                        "tracks": tracks.total,
                                        "public": public,
                                        "collaborative": collaborative,
                                        "owner": owner.display_name,
                                        "id": id,
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
            .route("/albume", web::get().to(get_saved_albums))
            .route("/artist", web::get().to(get_followed_artists))
            .route("/tracks", web::get().to(get_followed_tracks))
            .route("/playlist", web::get().to(get_followed_playlist))
            .route("/addTrackToPlaylist", web::get().to(search_and_add_tracks))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
