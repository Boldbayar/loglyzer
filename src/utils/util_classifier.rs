pub enum LogLevel {
    Error,
    Warning,
    Info,
}

pub fn classify(pattern: &str) -> LogLevel {
    let lower = pattern.to_lowercase();

    if lower.contains("exception") || lower.contains("fail") {
        LogLevel::Error
    } else if lower.contains("unauthorized")
        || lower.contains("timeout")
        || lower.contains("refused")
    {
        LogLevel::Warning
    } else {
        LogLevel::Info
    }
}
