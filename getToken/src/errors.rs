// 自定义处理错误类型
use actix_web::{HttpResponse, ResponseError, http::status_code};
use derive_more::Display; // 自定义错误显示
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _O)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "NotFound: {}", _O)]
    NotFound(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_id = Uuid::new_v4().to_string(); // 生成唯一的错误ID
        let error_message = self.to_string();

        // 根据错误类型或当前运行环境决定返回给客户端的错误描述
        let clien_error_messgae = match *self {
            AppError::InternalServerError => "Internal Server Error".to_string(),
            _ => error_message.clone(),
        };

        // 使用log记录
        log::error!("Error: {}, ErrorID: {}", error_message, error_id);

        // 构建错误响应
        let response_body = json!({
            "error": client_error_message,
            "error_id": error_id,
        });
        HttpResponse::build(self.status_code()).json(response_body)
    }
}