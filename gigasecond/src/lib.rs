use time::{OffsetDateTime, PrimitiveDateTime as DateTime};
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let unix = start.assume_utc().unix_timestamp() + 1_000_000_000;

    if let Ok(new_date) = OffsetDateTime::from_unix_timestamp(unix) {
        return DateTime::new(new_date.date(), new_date.time());
    }

    DateTime::MIN
}
