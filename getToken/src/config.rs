// 读取配置文件类型

use dotenv::dotenv;
use serde::Deserialize;
use std::env;


#[derive(Deserialize)]
pub struct AppConfig {
    pub spotify_client_id: String,
    pub spotify_client_secret: String,
    pub redirect_uri: String,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self, env::VarError> {
        dotenv().ok(); //加载 .env文件的内容

        // 使用'env::var'从环境中读取配置，并转换为'AppConfig'实例
        Ok(Self {
            spotify_client_id: env::var("SPOTIFY_CLIENT_ID")?,
            spotify_client_secret: env::var("SPOTIFY_CLIENT_SECRET")?,
            redirect_uri: env::var("REDIRECT_URI")?,
        })
    }
}