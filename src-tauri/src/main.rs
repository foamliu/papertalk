// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use reqwest;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[tauri::command]
async fn check_ollama() -> Result<bool, String> {
    let client = reqwest::Client::new();
    match client.get("http://127.0.0.1:11434/api/tags").send().await {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
async fn translate_text(text: String) -> Result<String, String> {
    println!("🌐 收到翻译请求，文本：{}", text);


    let client = reqwest::Client::new();
    let request = OllamaRequest {
        model: "qwen3:8b-q4_K_M".to_string(),
        prompt: format!("请将以下英文文本翻译成中文，保持专业术语不变：{} /no_think", text),
        stream: false,
    };

    let url = "http://127.0.0.1:11434/api/generate";
    println!("[translate_text] 请求 URL: {}", url);

    match client
        .post(url)
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                let ollama_resp: OllamaResponse = resp.json().await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;
                let re = Regex::new(r"(?s)<think>\s*.*?\s*</think>\s*").unwrap();
                let cleaned = re.replace_all(&ollama_resp.response, "").trim().to_string();
                println!("[translate_text] 翻译结果: {}", cleaned);
                Ok(cleaned)
            } else {
                println!("[translate_text] ❌ 非 2xx 状态：{}", resp.status());
                Err("Translation failed".to_string())
            }
        }
        Err(e) => {
            println!("[translate_text] ❌ 网络错误: {}", e);
            Err(format!("Network error: {}", e))
        }
    }
}

#[tauri::command]
fn open_ollama_website() -> Result<(), String> {
    let url = "https://ollama.ai";
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
    } else {
        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("Failed to open browser: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn open_file_dialog() -> Result<String, String> {
    use tauri::api::dialog::FileDialogBuilder;
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    FileDialogBuilder::new()
        .add_filter("PDF files", &["pdf"])
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });
    
    match rx.recv() {
        Ok(Some(path)) => {
            if let Some(path_str) = path.to_str() {
                // For Tauri, we need to convert the file path to a URL that PDF.js can access
                // We'll use a custom protocol handler
                Ok(format!("tauri://localhost/{}", path_str.replace("\\", "/")))
            } else {
                Err("Invalid file path".to_string())
            }
        }
        Ok(None) => Err("No file selected".to_string()),
        Err(_) => Err("Failed to receive file path".to_string()),
    }
}

#[tauri::command]
async fn get_pdf_data(path: String) -> Result<Vec<u8>, String> {
    use std::fs;
    
    println!("📁 Received request to read PDF: {}", path);
    
    // Remove the protocol prefix if present
    let clean_path = path.replace("tauri://localhost/", "");
    println!("📁 Cleaned path: {}", clean_path);
    
    match fs::read(&clean_path) {
        Ok(data) => {
            println!("✅ Successfully read PDF file, size: {} bytes", data.len());
            Ok(data)
        },
        Err(e) => {
            println!("❌ Failed to read PDF file: {}", e);
            Err(format!("Failed to read PDF file: {}", e))
        },
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_ollama,
            translate_text,
            open_ollama_website,
            open_file_dialog,
            get_pdf_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
