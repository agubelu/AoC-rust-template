#[cfg(windows)]
pub const DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
pub const DOUBLE_NEWLINE: &str = "\n\n";