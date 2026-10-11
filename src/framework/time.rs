use time::{OffsetDateTime, UtcOffset};

pub(crate) fn now() -> OffsetDateTime {
    let utc_datetime = OffsetDateTime::now_utc();
    let jst_offset = UtcOffset::from_hms(9, 0, 0).unwrap();
    utc_datetime.to_offset(jst_offset)
}
