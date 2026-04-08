use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct AnalysisResult {
    pub language: String,
    pub patterns: Vec<(String, usize)>,
    pub ai_summary: String,
}

pub fn save_result(result: &AnalysisResult) {
    let json = serde_json::to_string_pretty(result).unwrap();
    fs::write("analysis.json", json).expect("Failed to write file");
}
