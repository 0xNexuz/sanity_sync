# 📊 SanitySync: MCP
**Autonomous Data QA Layer for Google Workspace.**
*Built for the Google Cloud Rapid Agent Hackathon*

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Gemini 2.5](https://img.shields.io/badge/Gemini_2.5_Flash-%234285F4.svg?style=for-the-badge&logo=google&logoColor=white)](https://ai.google.dev/)
[![MCP](https://img.shields.io/badge/MCP-Enabled-success.svg)](#)

## ⚠️ The Problem: Human Error in Enterprise Data
Teams relying on spreadsheets for complex scientific modeling, financial indexing, or tokenomics constantly fight human error. A single missing negative sign or misaligned row can skew an entire dataset (e.g., ruining a Shannon-Weiner biodiversity index). Manual QA is slow, expensive, and error-prone.

## 🛠️ The Solution: Agentic Tool Calling
SanitySync is a lightweight Model Context Protocol (MCP) server written in Rust. It exposes specific deterministic tools (like `flag_anomaly`) to a Gemini-powered QA Agent. 

Instead of acting as a conversational chatbot, Gemini operates as a silent backend auditor. It ingests massive rows of spreadsheet data, autonomously checks the mathematical logic, and when it detects an anomaly, it formulates a structured JSON-RPC call demanding the Rust server physically update the sheet with a correction.

### How it Works
1. **Data Ingestion:** Rust fetches target data ranges.
2. **Cognitive Audit:** Gemini 2.5 Flash evaluates the logic (e.g., verifying `pi * ln_pi` calculations).
3. **Tool Execution:** If an error is found, Gemini triggers the `flag_anomaly` Rust function, passing the exact row, column, explanation, and auto-fix parameters.

## 🚀 Quick Start
```bash

###
Note: Ensure your .env contains a valid GEMINI_API_KEY.


***

### 3. Push to GitHub safely
You already ran `echo ".env" >> .gitignore`, so your new key is safe. 
Run the standard Git flow:

```bash
git init
git add .
git commit -m "Initial commit: Core MCP architecture and Gemini tool-calling"
Then create the sanity_sync repo on your GitHub and push the code:

Bash
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/sanity_sync.git
git push -u origin main
git clone [https://github.com/YOUR_GITHUB/sanity_sync.git](https://github.com/YOUR_GITHUB/sanity_sync.git)
cd sanity_sync
cargo run
