/// Parse estimate string to minutes.
/// Supported formats:
/// - "30m" or "30min" → 30 minutes
/// - "2h" or "2hr" or "2hrs" → 120 minutes
/// - "1h30m" or "1h 30m" → 90 minutes
/// - "1.5h" → 90 minutes
/// - Plain number treated as minutes: "30" → 30 minutes
pub fn parse_estimate(s: &str) -> Result<i64, String> {
    let s = s.trim().to_lowercase();

    if s.is_empty() {
        return Err("Estimate cannot be empty".to_string());
    }

    // Handle combined format like "1h30m" or "2h 15m"
    if s.contains('h') && s.contains('m') {
        let parts: Vec<&str> = s.split('h').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid estimate format: '{}'", s));
        }

        let hours: f64 = parts[0].trim().parse().map_err(|_| {
            format!("Invalid hours in estimate: '{}'", parts[0])
        })?;

        let min_str = parts[1].trim().trim_end_matches(|c| c == 'm' || c == 'i' || c == 'n');
        let mins: i64 = if min_str.is_empty() {
            0
        } else {
            min_str.parse().map_err(|_| {
                format!("Invalid minutes in estimate: '{}'", parts[1])
            })?
        };

        return Ok((hours * 60.0) as i64 + mins);
    }

    // Handle hours: "2h", "2hr", "2hrs", "1.5h"
    if s.ends_with('h') || s.ends_with("hr") || s.ends_with("hrs") {
        let num_str = s.trim_end_matches(|c| c == 'h' || c == 'r' || c == 's');
        let hours: f64 = num_str.parse().map_err(|_| {
            format!("Invalid hours: '{}'", s)
        })?;
        return Ok((hours * 60.0) as i64);
    }

    // Handle minutes: "30m", "30min"
    if s.ends_with('m') || s.ends_with("min") {
        let num_str = s.trim_end_matches(|c| c == 'm' || c == 'i' || c == 'n');
        let mins: i64 = num_str.parse().map_err(|_| {
            format!("Invalid minutes: '{}'", s)
        })?;
        return Ok(mins);
    }

    // Plain number treated as minutes
    s.parse::<i64>().map_err(|_| {
        format!(
            "Invalid estimate '{}'. Use formats like: 30m, 2h, 1h30m, 1.5h",
            s
        )
    })
}

/// Format minutes to human-readable string
pub fn format_estimate(minutes: Option<i64>) -> String {
    match minutes {
        None => "-".to_string(),
        Some(0) => "0m".to_string(),
        Some(m) if m < 60 => format!("{}m", m),
        Some(m) if m % 60 == 0 => format!("{}h", m / 60),
        Some(m) => format!("{}h{}m", m / 60, m % 60),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_estimate_minutes() {
        assert_eq!(parse_estimate("30m").unwrap(), 30);
        assert_eq!(parse_estimate("30min").unwrap(), 30);
        assert_eq!(parse_estimate("45m").unwrap(), 45);
        assert_eq!(parse_estimate("0m").unwrap(), 0);
    }

    #[test]
    fn test_parse_estimate_hours() {
        assert_eq!(parse_estimate("1h").unwrap(), 60);
        assert_eq!(parse_estimate("2h").unwrap(), 120);
        assert_eq!(parse_estimate("2hr").unwrap(), 120);
        assert_eq!(parse_estimate("2hrs").unwrap(), 120);
        assert_eq!(parse_estimate("1.5h").unwrap(), 90);
        assert_eq!(parse_estimate("0.5h").unwrap(), 30);
    }

    #[test]
    fn test_parse_estimate_combined() {
        assert_eq!(parse_estimate("1h30m").unwrap(), 90);
        assert_eq!(parse_estimate("2h15m").unwrap(), 135);
        assert_eq!(parse_estimate("1h 30m").unwrap(), 90);
        assert_eq!(parse_estimate("0h30m").unwrap(), 30);
    }

    #[test]
    fn test_parse_estimate_plain_number() {
        assert_eq!(parse_estimate("30").unwrap(), 30);
        assert_eq!(parse_estimate("60").unwrap(), 60);
        assert_eq!(parse_estimate("120").unwrap(), 120);
    }

    #[test]
    fn test_parse_estimate_case_insensitive() {
        assert_eq!(parse_estimate("2H").unwrap(), 120);
        assert_eq!(parse_estimate("30M").unwrap(), 30);
        assert_eq!(parse_estimate("1H30M").unwrap(), 90);
    }

    #[test]
    fn test_parse_estimate_invalid() {
        assert!(parse_estimate("").is_err());
        assert!(parse_estimate("abc").is_err());
        assert!(parse_estimate("1x").is_err());
    }

    #[test]
    fn test_format_estimate() {
        assert_eq!(format_estimate(None), "-");
        assert_eq!(format_estimate(Some(0)), "0m");
        assert_eq!(format_estimate(Some(30)), "30m");
        assert_eq!(format_estimate(Some(60)), "1h");
        assert_eq!(format_estimate(Some(90)), "1h30m");
        assert_eq!(format_estimate(Some(120)), "2h");
        assert_eq!(format_estimate(Some(135)), "2h15m");
    }
}
