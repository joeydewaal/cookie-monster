pub fn parse_max_age(mut attribute: &str) -> Option<u64> {
    if attribute.is_empty() {
        return None;
    }

    // If the first character of the attribute-value is not a DIGIT or a "-"
    // character, ignore the cookie-av.
    let is_negative = match attribute.as_bytes()[0] {
        b'-' => {
            attribute = &attribute[1..];
            true
        }
        _ => false,
    };

    // If the remainder of attribute-value contains a non-DIGIT character, ignore the cookie-av.
    if !attribute.chars().all(|char| char.is_ascii_digit()) {
        return None;
    }

    // If delta-seconds is less than or equal to zero (0), let expiry-time
    // be the earliest representable date and time.  Otherwise, let the
    // expiry-time be the current date and time plus delta-seconds seconds.
    if is_negative {
        Some(0)
    } else {
        attribute.parse::<u64>().ok()
    }
}
