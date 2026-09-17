#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Response(qtest::Response);

impl From<qtest::Response> for Response {
    fn from(resp: qtest::Response) -> Self {
        Response(resp)
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            qtest::Response::Ok | qtest::Response::OkVal(_) => axum::http::StatusCode::OK,
            qtest::Response::Err(_) => axum::http::StatusCode::BAD_REQUEST,
        };
        let body = axum::body::Body::from(self.to_string());
        axum::response::Response::builder()
            .status(status)
            .body(body)
            .unwrap()
    }
}

#[derive(Debug)]
pub enum Error {
    QemuClosed,
    Io(std::io::Error),
    Json(serde_json::Error),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::QemuClosed => write!(f, "QEMU session is closed"),
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
            Error::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Error::QemuClosed => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Error::Io(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::Json(_) | Error::Other(_) => axum::http::StatusCode::BAD_REQUEST,
        };
        let body = axum::body::Body::from(self.to_string());
        axum::response::Response::builder()
            .status(status)
            .body(body)
            .unwrap()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
