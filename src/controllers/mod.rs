#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    message: String,
    code: u64
}

impl Error {
    pub fn new(message: String, code: u64) -> Error {
        Error { message: message, code: code }
    }
}

// Error構造体に変換するために
pub trait ToError {
    fn to_error(&self) -> Error;
}

// レスポンスの基本型
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<Error>
}

// 成功時
impl <T> ResponseBase<T> {
    pub fn success(data: T) -> ResponseBase<T> {
        ResponseBase {
            success: true,
            data: Some(data),
            error: None
        }
    }
}

// ToErrorとErrorでfailするためのトレイト
pub trait Fail<T, E> {
    fn fail(error: E) -> ResponseBase<T>;
}

impl <T, E> Fail<T, E> for ResponseBase<T> where E: ToError {
    fn fail(error: E) -> ResponseBase<T> {
        ResponseBase {
            success: false,
            data: None,
            error: Some(error.to_error())
        }
    }
}

impl <T> Fail<T, Error> for ResponseBase<T> {
    fn fail(error: Error) -> ResponseBase<T> {
        ResponseBase {
            success: false,
            data: None,
            error: Some(error)
        }
    }
}

impl ToError for () {
    fn to_error(&self) -> Error {
        Error { message: "".to_string(), code: 0}
    }
}

pub mod access_tokens;
pub mod articles;
pub mod users;
