use reqwest::{Client, Response};
use serde_json::json;
use std::error::Error;

pub async fn analyze_logs(logs: String, api_key: String) -> Result<String, Box<dyn Error>> {
    let client: Client = Client::new();

    let prompt = format!(
        "You are a senior backend engineer.

        Analyze the following categorized system logs.

        Focus on:
        - identifying root causes
        - detecting abnormal patterns
        - understanding system behavior

        Return your answer in this structure:

        ### 1. Summary
        - What is happening overall?

        ### 2. Key Issues
        - What are the most critical problems?

        ### 3. Root Causes
        - Why are these happening?

        ### 4. Suggestions
        - What should be fixed or improved?

        ### Logs:
        {}",
        logs
    );

    let res: Response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "gpt-4o-mini",
            "temperature": 0.3,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }))
        .send()
        .await?;

    let body: serde_json::Value = res.json().await?;

    let output =
        body["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();

    Ok(output)
}
