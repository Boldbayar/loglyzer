pub fn detect_language(logs: &str) -> String {
    let mut scores = std::collections::HashMap::new();

    let java_keywords = ["NullPointerException", "at ", "Exception:", "Caused by:", ".java:"];
    let python_keywords = ["Traceback (most recent call last)", "File \"", "line ", "Exception:"];
    let js_keywords = ["TypeError", "ReferenceError", "at ", ".js:", "UnhandledPromiseRejection"];
    let go_keywords = ["panic:", "goroutine", ".go:"];

    fn score(logs: &str, keywords: &[&str]) -> usize {
        keywords.iter().filter(|k| logs.contains(*k)).count()
    }

    scores.insert("Java", score(logs, &java_keywords));
    scores.insert("Python", score(logs, &python_keywords));
    scores.insert("JavaScript", score(logs, &js_keywords));
    scores.insert("Go", score(logs, &go_keywords));

    let (lang, max_score) = scores.into_iter().max_by_key(|(_, score)| *score).unwrap();
    if max_score == 0 { "Unknown".to_string() } else { lang.to_string() }
}
