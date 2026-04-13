use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct AnalysisResult {
    pub language: String,
    pub patterns: Vec<(String, usize)>,
    pub ai_summary: String,
}

pub fn save_result(result: &AnalysisResult) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(result).unwrap();
    fs::write("analysis.json", json).expect("Failed to write file");
    Ok(())
}
