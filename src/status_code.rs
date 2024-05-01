pub(crate) enum StatusCode {
    Ok,
    Created,
    NotFound,
    InternalServerError,
    Conflict,
}

impl StatusCode {
    pub fn to_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::NotFound => 404,
            StatusCode::Conflict => 409,
            StatusCode::InternalServerError => 500
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => "OK".to_string(),
            StatusCode::Created => "Created".to_string(),
            StatusCode::NotFound => "Not Found".to_string(),
            StatusCode::Conflict => "Conflict".to_string(),
            StatusCode::InternalServerError => "Internal Server Error".to_string(),
        }
    }
}