use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

pub fn section(title: &str) {
    println!("\n{}", format!("━━ {} ━━", title).bold().blue());
}

pub fn success(msg: &str) {
    println!("{}", format!("✔ {}", msg).green());
}

pub fn warning(msg: &str) {
    println!("{}", format!("⚠ {}", msg).yellow());
}

pub fn error(msg: &str) {
    println!("{}", format!("✖ {}", msg).red());
}

pub fn info(msg: &str) {
    println!("{}", format!("➜ {}", msg).cyan());
}

pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());

    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_message(msg.to_string());

    pb
}
