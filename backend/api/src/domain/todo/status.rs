use std::str::FromStr;

pub enum Status {
    Ready,
    Active,
    Completed,
    Pending,
}

impl Status {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Ready" => Ok(Status::Ready),
            "Active" => Ok(Status::Active),
            "Completed" => Ok(Status::Completed),
            "Pending" => Ok(Status::Pending),
            _ => Err("aaa".to_string()),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Ready => "Ready",
            Status::Active => "Active",
            Status::Completed => "Completed",
            Status::Pending => "Pending",
        }
    }
}
