pub(crate) enum StatusCode {
    Ok,
    NotFound,
    InternalServerError
}

impl StatusCode {
    pub fn to_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => "OK".to_string(),
            StatusCode::NotFound => "Not Found".to_string(),
            StatusCode::InternalServerError => "Internal Server Error".to_string()
        }
    }
}