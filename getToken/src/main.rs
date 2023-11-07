use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::header};
use serde::{Deserialize, Serialize};
use reqwest::Client;

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
    let scope = "user-read-private user-read-email";
    let spotify_url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}",
        client_id, scope, redirect_uri
    );
    HttpResponse::Found().append_header((header::LOCATION, spotify_url)).finish()
}

async fn spotify_callback(info: web::Query<AuthQuery>) -> impl Responder {
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

    match client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await
    {
        Ok(resp) => match resp.json::<AuthTokenResponse>().await {
            Ok(auth_token_response) => HttpResponse::Ok().json(auth_token_response),
            Err(_) => HttpResponse::BadRequest().body("Failed to parse response"),
        },
        Err(_) => HttpResponse::BadRequest().body("Failed to send request"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(spotify_login))
            .route("/callback", web::get().to(spotify_callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
