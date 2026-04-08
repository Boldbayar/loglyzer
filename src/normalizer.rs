use serde_json::Value;
use std::collections::HashMap;

pub fn normalize_logs(logs: &str, level: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    for line in logs.lines() {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() >= 3 {
            let json_part = parts[2];

            if let Ok(parsed) = serde_json::from_str::<Value>(json_part) {
                let severity = parsed["severity"].as_str().unwrap_or("").to_lowercase();

                let should_include = match level {
                    "error" => severity == "error",
                    "info" => severity == "info",
                    "all" => true,
                    _ => severity == "error",
                };
                if !should_include {
                    continue;
                }

                if let Some(body) = parsed["body"].as_str() {
                    process_line(body, &mut map);
                    continue;
                }
            }
        }

        let lower = line.to_lowercase();
        if lower.contains("error") || lower.contains("exception") || lower.contains("fail") {
            process_line(line, &mut map);
        }
    }

    map
}

fn process_line(line: &str, map: &mut HashMap<String, usize>) {
    let normalized = normalize_line(line);
    let root = extract_root(&normalized);
    if root.len() < 8 {
        return;
    }
    let word_count = root.split_whitespace().count();
    if word_count < 3 {
        return;
    }
    if is_noise(line) {
        return;
    }

    let count = map.entry(root).or_insert(0);
    *count += 1;
}

fn is_noise(line: &str) -> bool {
    let lower = line.to_lowercase();

    lower.contains("request begin")
        || lower.contains("request end")
        || lower.contains("headers")
        || lower.contains("body")
        || lower.contains("emitting")
}

fn normalize_line(line: &str) -> String {
    let mut result = Vec::new();

    for token in line.split_whitespace() {
        let cleaned = if is_dynamic(token) { "X" } else { token };

        result.push(cleaned);
    }

    let mut cleaned = result.join(" ");
    if let Some(pos) = cleaned.find(':') {
        cleaned = cleaned[..pos].to_string();
    }

    cleaned = cleaned.trim().to_string();
    if cleaned.len() > 80 {
        cleaned = cleaned.chars().take(80).collect();
    }

    cleaned
}

fn extract_root(line: &str) -> String {
    let lower = line.to_lowercase();

    if lower.contains("timeout") {
        return "TimeoutError".to_string();
    }
    if lower.contains("connection refused") {
        return "ConnectionRefused".to_string();
    }
    if lower.contains("failed") {
        return "OperationFailed".to_string();
    }

    if let Some(pos) = line.find("Exception") {
        let start = line[..pos]
            .rfind(|c: char| c == '.' || c == ' ' || c == ':')
            .map(|i| i + 1)
            .unwrap_or(0);

        return line[start..pos + "Exception".len()].to_string();
    }

    line.split_whitespace().filter(|w| *w != "X").take(5).collect::<Vec<_>>().join(" ")
}

fn is_dynamic(token: &str) -> bool {
    if token.chars().all(|c| c.is_numeric()) {
        return true;
    }

    if token.len() > 10 && token.chars().any(|c| c.is_numeric()) {
        return true;
    }

    if token.chars().filter(|c| c.is_numeric()).count() > 3 {
        return true;
    }

    if token.contains("T") && token.contains(":") {
        return true;
    }
    false
}
