use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: impl  Into<String>) -> Self {
        Self {
            data,
            message: Some(message.into()),
        }
    }
}