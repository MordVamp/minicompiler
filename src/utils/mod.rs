/// Utility helpers shared across compiler stages.

/// Format a compiler diagnostic message with location context.
pub fn format_diagnostic(level: &str, line: usize, column: usize, msg: &str) -> String {
    format!("[{}] {}:{}: {}", level, line, column, msg)
}

/// Format a rich error message with code snippet and a caret (^) pointing to the error.
pub fn format_error_with_context(source: &str, line: usize, column: usize, msg: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = format!("Error: {}\n", msg);
    result.push_str(&format!("  line {}, col {}:\n", line, column));
    
    if line > 0 && line <= lines.len() {
        let code_line = lines[line - 1];
        result.push_str(&format!("  {}\n", code_line));
        
        // Add caret pointer
        result.push_str("  ");
        for _ in 0..(column - 1) {
            result.push(' ');
        }
        result.push_str("^\n");
    }
    
    result
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
