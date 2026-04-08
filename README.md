# 🚀 Loglyzer

**Loglyzer** is a fast CLI tool written in Rust that analyzes log files, extracts meaningful patterns, and generates intelligent summaries using AI.

## ✨ Features
- 📂 Read multiple `.log` files from a folder
- 🧠 Smart log normalization
- 🧩 Pattern grouping
- 🚦 Classification (Errors / Warnings / System Activity)
- 🤖 AI-powered analysis
- ⚡ Fast native Rust CLI

## 📦 Installation
```bash
curl https://sh.rustup.rs -sSf | sh
git clone <your-repo-url>
cd loglyzer
cargo build --release
```

## 🔑 Setup
Create a `.env` file:
```
OPENAI_API_KEY=your_api_key_here
```

## 🚀 Usage
```bash
cargo run -- ./logs --level all
```

## ⚙️ CLI Options
| Option | Description |
|--------|------------|
| folder | Path to logs folder |
| --level | error / info / all |

## 📊 Example Output
```
━━ Warnings ━━
966x Unauthorized

━━ Errors ━━
24x NullValueError

━━ System Activity ━━
376x Expired payment invoices
128x Verify payment
```

## 🧠 AI Analysis
Provides:
- Summary
- Key Issues
- Root Causes
- Suggestions

## 🏗️ How It Works
logs → normalize → extract → categorize → AI

## 🔮 Roadmap
- Stacktrace grouping
- ONNX models
- Real-time streaming

## 🧾 License
MIT
