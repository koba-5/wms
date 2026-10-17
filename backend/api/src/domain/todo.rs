use crate::domain::todo::status::Status;

pub mod status;

pub struct Todo {
    pub id: i64,
    pub title: String,
    pub status: Status,
    pub start_date: String,
    pub end_date: Option<String>,
}

impl Todo {
    pub fn new(
        id: i64,
        title: String,
        status: Status,
        start_date: String,
        end_date: Option<String>,
    ) -> Self {
        Self {
            id,
            title,
            status,
            start_date,
            end_date,
        }
    }

    pub fn get(id: i64) -> Result<Self, String> {
        Ok(Self::new(
            1,
            "".to_string(),
            Status::Pending,
            "".to_string(),
            Some("".to_string()),
        ))
    }

    pub fn completed(&self) {}
}
