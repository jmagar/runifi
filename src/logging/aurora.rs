/// Aurora ANSI-256 color palette constants.
///
/// These match `lab/crates/lab/src/output/theme.rs` exactly so that
/// rustifi console logs are visually consistent with the rest of the
/// homelab tooling.
pub const SERVICE_NAME: u8 = 211; // pink        (255,175,215)
pub const ACCENT_PRIMARY: u8 = 39; // bright blue (41,182,246)
pub const TEXT_MUTED: u8 = 250; // light grey  (167,188,201)
pub const SUCCESS: u8 = 115; // teal        (125,211,199)
pub const WARN: u8 = 180; // amber       (198,163,107)
pub const ERROR: u8 = 174; // muted red   (199,132,144)

/// Wrap `text` in an ANSI-256 foreground color escape, bold.
#[must_use]
pub fn ansi256_bold(code: u8, text: &str) -> String {
    format!("\x1b[1m\x1b[38;5;{code}m{text}\x1b[0m")
}

/// Wrap `text` in an ANSI-256 foreground color escape (no bold).
#[must_use]
pub fn ansi256(code: u8, text: &str) -> String {
    format!("\x1b[38;5;{code}m{text}\x1b[0m")
}
