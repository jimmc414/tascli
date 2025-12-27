/// Parse priority string to numeric value.
/// Returns: 0 = high, 1 = normal, 2 = low
pub fn parse_priority(s: &str) -> Result<u8, String> {
    match s.to_lowercase().as_str() {
        "high" | "h" | "0" => Ok(0),
        "normal" | "n" | "1" | "med" | "medium" => Ok(1),
        "low" | "l" | "2" => Ok(2),
        _ => Err(format!(
            "Invalid priority '{}'. Use high/normal/low (or h/n/l)",
            s
        )),
    }
}

/// Format priority value to human-readable string
pub fn format_priority(priority: Option<u8>) -> &'static str {
    match priority {
        Some(0) => "HIGH",
        Some(1) => "normal",
        Some(2) => "low",
        _ => "-",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_priority() {
        // High priority
        assert_eq!(parse_priority("high").unwrap(), 0);
        assert_eq!(parse_priority("HIGH").unwrap(), 0);
        assert_eq!(parse_priority("h").unwrap(), 0);
        assert_eq!(parse_priority("H").unwrap(), 0);
        assert_eq!(parse_priority("0").unwrap(), 0);

        // Normal priority
        assert_eq!(parse_priority("normal").unwrap(), 1);
        assert_eq!(parse_priority("NORMAL").unwrap(), 1);
        assert_eq!(parse_priority("n").unwrap(), 1);
        assert_eq!(parse_priority("N").unwrap(), 1);
        assert_eq!(parse_priority("med").unwrap(), 1);
        assert_eq!(parse_priority("medium").unwrap(), 1);
        assert_eq!(parse_priority("1").unwrap(), 1);

        // Low priority
        assert_eq!(parse_priority("low").unwrap(), 2);
        assert_eq!(parse_priority("LOW").unwrap(), 2);
        assert_eq!(parse_priority("l").unwrap(), 2);
        assert_eq!(parse_priority("L").unwrap(), 2);
        assert_eq!(parse_priority("2").unwrap(), 2);

        // Invalid
        assert!(parse_priority("invalid").is_err());
        assert!(parse_priority("urgent").is_err());
    }

    #[test]
    fn test_format_priority() {
        assert_eq!(format_priority(Some(0)), "HIGH");
        assert_eq!(format_priority(Some(1)), "normal");
        assert_eq!(format_priority(Some(2)), "low");
        assert_eq!(format_priority(None), "-");
        assert_eq!(format_priority(Some(99)), "-");
    }
}
