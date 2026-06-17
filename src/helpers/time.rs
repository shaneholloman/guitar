use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use git2::Time;

pub fn timestamp_to_utc(time: Time) -> String {
    // Git stores the author's offset separately from epoch seconds.
    let offset = FixedOffset::east_opt(time.offset_minutes() * 60).unwrap();

    // Start with the raw epoch time, then apply the stored offset.
    let utc_datetime = DateTime::from_timestamp(time.seconds(), 0).expect("Invalid timestamp");

    // Normalize to UTC so inspector timestamps use one stable display timezone.
    let local_datetime = offset.from_utc_datetime(&utc_datetime.naive_utc());
    let final_utc: DateTime<Utc> = local_datetime.with_timezone(&Utc);

    final_utc.to_rfc2822()
}

pub fn timestamp_to_utc_date(time: Time) -> String {
    let offset = FixedOffset::east_opt(time.offset_minutes() * 60).unwrap();
    let utc_datetime = DateTime::from_timestamp(time.seconds(), 0).expect("Invalid timestamp");
    let local_datetime = offset.from_utc_datetime(&utc_datetime.naive_utc());
    let final_utc: DateTime<Utc> = local_datetime.with_timezone(&Utc);

    final_utc.format("%Y-%m-%d").to_string()
}
