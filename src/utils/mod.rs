/// Utility helpers shared across compiler stages.

/// Format a compiler diagnostic message with location context.
pub fn format_diagnostic(level: &str, line: usize, column: usize, msg: &str) -> String {
    format!("[{}] {}:{}: {}", level, line, column, msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_diagnostic() {
        let s = format_diagnostic("ERROR", 3, 12, "unexpected token");
        assert_eq!(s, "[ERROR] 3:12: unexpected token");
    }
}
