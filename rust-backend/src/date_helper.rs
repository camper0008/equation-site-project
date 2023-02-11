use chrono::prelude::Utc;

pub fn utc_date_iso_string() -> String {
    Utc::now().to_rfc3339()
}
