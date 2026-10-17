use crate::domain;
use crate::service::todo::get_response::GetResponse;
use crate::util::error::Error;
use axum::http::StatusCode;

mod get_response;

pub fn get(id: i64) -> Result<GetResponse, Error> {
    let a = domain::todo::Todo::get(id);

    match a {
        Ok(v) => Ok(GetResponse::new(
            v.id,
            v.title,
            v.status.as_str().to_string(),
            v.start_date,
            v.end_date,
        )),
        Err(_) => Err(Error::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "エラー".to_string(),
        )),
    }
}
