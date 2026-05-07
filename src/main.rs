use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio::time::{sleep, Duration};

// --- 1. MOCK DATA (With Bio-Logic Traps) ---
// We simulate a live Google Sheet feed from an Ikpoba River hydrobiology study.
// It contains two fatal errors: an arithmetic break, and a biological impossibility.
fn fetch_live_workspace_data() -> serde_json::Value {
    json!([
        {
            "row": 2,
            "station": "Upper Lawani Reservoir",
            "species": "Tilapia mariae",
            "pi": 0.5,
            "ln_pi": -0.693,
            "pi_ln_pi": 0.3465 // ARITHMETIC ERROR: Missing negative sign
        },
        {
            "row": 3,
            "station": "Guinness Outfall",
            "sample_type": "Benthic water",
            "pH_level": 14.5, // BIOLOGICAL ERROR: Impossible pH for a freshwater river
            "dissolved_oxygen_mgL": -1.2 // BIOLOGICAL ERROR: Impossible negative DO
        }
    ])
}

// --- 2. OUR LOCAL RUST TOOL (The Audit Trail Reflex) ---
// This simulates generating a new tab in Google Sheets to log all anomalies.
fn execute_generate_audit_report(anomalies: &serde_json::Value) {
    println!("\n📑 [MCP ACTION EXECUTED: generate_audit_report]");
    println!("   ├── 📁 Creating new Workspace tab: 'SanitySync_Audit_Log_Auto'");
    
    if let Some(arr) = anomalies.as_array() {
        for (i, anomaly) in arr.iter().enumerate() {
            let row = anomaly["row"].as_u64().unwrap_or(0);
            let category = anomaly["category"].as_str().unwrap_or("UNKNOWN");
            let issue = anomaly["issue_description"].as_str().unwrap_or("No description");
            let fix = anomaly["recommended_action"].as_str().unwrap_or("Review manually");
            
            println!("   ├── ⚠️  Anomaly {} | Row {} [{}]", i + 1, row, category);
            println!("   │    ├─ Issue: {}", issue);
            println!("   │    └─ Action: {}", fix);
        }
    }
    println!("   └── ✅ Audit Trail successfully synced to Google Drive.\n");
}

// --- 3. GEMINI API WRAPPER STRUCTS ---
#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}
#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: GeminiContent,
}
#[derive(Deserialize, Debug)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}
#[derive(Deserialize, Debug)]
struct GeminiPart {
    #[serde(rename = "functionCall")]
    function_call: Option<FunctionCall>,
    text: Option<String>,
}
#[derive(Deserialize, Debug)]
struct FunctionCall {
    name: String,
    args: serde_json::Value,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("🚨 GEMINI_API_KEY not found!");
    let client = Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);

    println!("🚀 SanitySync Enterprise MCP Server Initialized...");
    
    // --- FEATURE: WATCHDOG MODE ---
    // Wraps the entire execution in an asynchronous cron-like loop
    println!("🐕 [WATCHDOG] Entering background polling mode. Listening for Workspace changes...");

    // We use a loop to simulate continuous polling. (It will run once, sleep, and run again).
    loop {
        println!("\n---------------------------------------------------");
        println!("🔄 [WATCHDOG] Polling cycle initiated...");
        println!("📊 Fetching Workspace Data: Ikpoba River Field Reports...");
        
        let sheet_data = fetch_live_workspace_data();
        println!("🧠 Routing data to Agentic QA Core for cross-referencing...");

        // --- 4. THE MCP TOOL DEFINITION (Audit Trail Schema) ---
        let tools = json!([{
            "functionDeclarations": [
                {
                    "name": "generate_audit_report",
                    "description": "Generates a comprehensive audit trail, logging all arithmetic and biological anomalies found in the dataset.",
                    "parameters": {
                        "type": "OBJECT",
                        "properties": {
                            "anomalies": {
                                "type": "ARRAY",
                                "description": "A list of all errors found in the sheet.",
                                "items": {
                                    "type": "OBJECT",
                                    "properties": {
                                        "row": {"type": "INTEGER"},
                                        "category": {"type": "STRING", "description": "'ARITHMETIC' or 'BIOLOGICAL_IMPOSSIBILITY'"},
                                        "issue_description": {"type": "STRING"},
                                        "recommended_action": {"type": "STRING"}
                                    }
                                }
                            }
                        },
                        "required": ["anomalies"]
                    }
                }
            ]
        }]);

        // --- 5. THE MASTER PROMPT (Bio-Logic Engine) ---
        let prompt_text = format!(
            "You are an autonomous data-sanity agent for Google Sheets, auditing hydrobiology field data. \
            Review the following dataset: {}. \
            \
            You must check for TWO things: \
            1. Arithmetic logic (e.g., verify Shannon-Weiner calculations like pi * ln_pi). \
            2. Biological/Physical reality (e.g., freshwater pH cannot be 14.5, dissolved oxygen cannot be negative). \
            \
            If you find ANY errors, batch them together and call the 'generate_audit_report' function to log them. \
            Do not provide conversational text. Only use the tool.",
            sheet_data
        );

        let payload = json!({
            "contents": [{"parts": [{"text": prompt_text}]}],
            "tools": tools,
        });

        // --- 6. EXECUTE AND PARSE ---
        let res = client.post(&url).json(&payload).send().await;

        match res {
            Ok(response) => {
                let raw_text = response.text().await.unwrap_or_default();
                
                if let Ok(gemini_data) = serde_json::from_str::<GeminiResponse>(&raw_text) {
                    if let Some(candidate) = gemini_data.candidates.first() {
                        if let Some(part) = candidate.content.parts.first() {
                            
                            // Check if the AI decided to call our Audit tool
                            if let Some(func_call) = &part.function_call {
                                if func_call.name == "generate_audit_report" {
                                    let anomalies = &func_call.args["anomalies"];
                                    execute_generate_audit_report(anomalies);
                                }
                            } else if let Some(text_response) = &part.text {
                                println!("💬 [AGENT MESSAGE]: {}", text_response);
                            }
                        }
                    }
                } else {
                    println!("⚠️ Failed to parse API response.");
                }
            },
            Err(e) => println!("⚠️ Network error: {}", e),
        }
        
        println!("🏁 QA cycle complete.");
        
        // Watchdog sleep state (Set to 15 seconds for demo purposes. In production, this would be hours).
        println!("💤 [WATCHDOG] Sleeping for 15 seconds before next polling cycle...");
        sleep(Duration::from_secs(15)).await;
    }
}