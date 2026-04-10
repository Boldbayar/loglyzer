mod payload;

mod cli;
mod services;
mod ui;
mod utils;

use services::service_analyzer::analyze_logs;
use services::service_normalizer::normalize_logs;

use colored::Colorize;
use dotenvy::dotenv;
use std::env;
use tracing::{error, warn};
use tracing_subscriber;
use ui::{error as ui_error, *};

use utils::util_classifier::{LogLevel, classify};
use utils::util_file_loader::read_logs_from_folder;
use utils::util_file_writer::{AnalysisResult, save_result};
use utils::util_lang_detector::detect_language;

use clap::Parser;
use cli::cli_args::Args;
use payload::models::Categorized;

fn initialize() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
}

fn categorize_patterns(patterns: &[(String, usize)]) -> Categorized {
    let mut result = Categorized { errors: vec![], warnings: vec![], infos: vec![] };

    for (pattern, count) in patterns.iter().take(10) {
        match classify(pattern) {
            LogLevel::Error => result.errors.push((pattern.clone(), *count)),
            LogLevel::Warning => result.warnings.push((pattern.clone(), *count)),
            LogLevel::Info => result.infos.push((pattern.clone(), *count)),
        }
    }
    result
}

fn print_categories(cat: &Categorized) {
    if !cat.errors.is_empty() {
        section("Errors");
        for (p, c) in &cat.errors {
            println!("{} {}", format!("{:>4}x", c).red().bold(), p);
        }
    }

    if !cat.warnings.is_empty() {
        section("Warnings");
        for (p, c) in &cat.warnings {
            println!("{} {}", format!("{:>4}x", c).yellow().bold(), p);
        }
    }

    if !cat.infos.is_empty() {
        section("System Activity");
        for (p, c) in &cat.infos {
            println!("{} {}", format!("{:>4}x", c).cyan(), p);
        }
    }
}

#[tokio::main]
async fn main() {
    initialize();

    let args = Args::parse();

    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");

    // ── Load logs ─────────────────────────────
    section("Loading Logs");

    let content = read_logs_from_folder(&args.folder);

    if content.is_empty() {
        warn!("No log files found");
        warning("No log files found");
        return;
    }

    success("Logs loaded from folder");

    // ── Detect language ───────────────────────
    section("Environment");

    let lang = detect_language(&content);
    info(&format!("Detected language: {}", lang));

    // ── Extract patterns ──────────────────────
    section("Log Analysis");

    let patterns_map = normalize_logs(&content, &args.level);
    let mut patterns: Vec<(String, usize)> =
        patterns_map.iter().map(|(p, c)| (p.clone(), *c)).collect();

    if patterns.is_empty() {
        warn!("No patterns found");
        warning("No error patterns found");
        return;
    }

    patterns.sort_by(|a, b| b.1.cmp(&a.1));
    let categorized = categorize_patterns(&patterns);
    print_categories(&categorized);

    // ── Prepare summary ───────────────────────
    let summary = format!(
        "Errors:\n{}\n\nWarnings:\n{}\n\nSystem Activity:\n{}\n",
        categorized
            .errors
            .iter()
            .map(|(p, c)| format!("{}: {}", c, p))
            .collect::<Vec<_>>()
            .join("\n"),
        categorized
            .warnings
            .iter()
            .map(|(p, c)| format!("{}: {}", c, p))
            .collect::<Vec<_>>()
            .join("\n"),
        categorized
            .infos
            .iter()
            .map(|(p, c)| format!("{}: {}", c, p))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    // ── AI analysis ───────────────────────────
    section("AI Analysis");

    let spinner = spinner("Analyzing logs with AI...");

    match analyze_logs(summary, api_key).await {
        Ok(ai_output) => {
            spinner.finish_and_clear();

            success("Analysis complete");

            println!("\n{}", ai_output);

            let analysis = AnalysisResult {
                language: lang,
                patterns: patterns.iter().take(5).map(|(p, c)| ((*p).clone(), *c)).collect(),
                ai_summary: ai_output,
            };

            save_result(&analysis);

            success("Saved to analysis.json");
        }
        Err(e) => {
            spinner.finish_and_clear();

            error!(error = %e, "AI analysis failed");
            ui_error(&format!("AI analysis failed: {}", e));
        }
    }
}
