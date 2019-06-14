#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T> {
    pub success: bool,
    pub data: T
}

impl <T> ResponseBase<T> {
    pub fn new(success: bool, data: T) -> ResponseBase<T> {
        ResponseBase {
            success: success,
            data: data
        }
    }
    
    pub fn success(data: T) -> ResponseBase<T> {
        ResponseBase::new(true, data)
    }

    pub fn fail(data: T) -> ResponseBase<T> {
        ResponseBase::new(false, data)
    }
}

pub mod access_tokens;
pub mod articles;
pub mod users;
