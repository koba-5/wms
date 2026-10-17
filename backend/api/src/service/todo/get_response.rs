use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub(super) struct GetResponse {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

impl GetResponse {
    pub(super) fn new(
        id: i64,
        title: String,
        status: String,
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
}
