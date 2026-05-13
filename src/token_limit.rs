/// Maximum response size before truncation.
///
/// Approximately 10,000 tokens at typical JSON density.
pub const MAX_RESPONSE_BYTES: usize = 40_000;

/// Truncate a serialized response if it exceeds [`MAX_RESPONSE_BYTES`].
///
/// Appends a human-readable hint that tells the agent to use `limit`/`offset`
/// or more specific filters to retrieve the full dataset.
#[must_use]
pub fn truncate_response(text: String) -> String {
    if text.len() <= MAX_RESPONSE_BYTES {
        return text;
    }
    // Find last valid UTF-8 boundary at/before the cap.
    let mut boundary = MAX_RESPONSE_BYTES;
    while !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    format!(
        "{}\n\n[TRUNCATED: response exceeded 10K token limit (~40 KB). \
         Use limit/offset or more specific filters to narrow the result set.]",
        &text[..boundary]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_response_passes_through() {
        let s = "hello".to_string();
        assert_eq!(truncate_response(s.clone()), s);
    }

    #[test]
    fn long_response_is_truncated() {
        let s = "x".repeat(MAX_RESPONSE_BYTES + 100);
        let result = truncate_response(s);
        assert!(result.contains("[TRUNCATED:"));
        assert!(result.len() < MAX_RESPONSE_BYTES + 200);
    }

    #[test]
    fn exactly_at_limit_passes() {
        let s = "y".repeat(MAX_RESPONSE_BYTES);
        let result = truncate_response(s.clone());
        assert_eq!(result, s);
    }
}
