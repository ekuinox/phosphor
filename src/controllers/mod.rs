#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T, E> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<E>
}

impl <T, E> ResponseBase<T, E> {
    pub fn new(success: bool, data: Option<T>, error: Option<E>) -> ResponseBase<T, E> {
        ResponseBase {
            success: success,
            data: data,
            error: error
        }
    }
    
    pub fn success(data: T) -> ResponseBase<T, E> {
        ResponseBase::new(true, Some(data), None)
    }

    pub fn fail(error: E) -> ResponseBase<T, E> {
        ResponseBase::new(false, None, Some(error))
    }
}

pub mod access_tokens;
pub mod articles;
pub mod users;
