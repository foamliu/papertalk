// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use reqwest;

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
    let client = reqwest::Client::new();
    let request = OllamaRequest {
        model: "qwen3:8b-q4_K_M".to_string(),
        prompt: format!("请将以下英文文本翻译成中文，保持专业术语不变：{}", text),
        stream: false,
    };

    match client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let ollama_response: OllamaResponse = response.json().await
                    .map_err(|e| format!("Failed to parse response: {}", e))?;
                Ok(ollama_response.response)
            } else {
                Err("Translation failed".to_string())
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
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
                Ok(path_str.to_string())
            } else {
                Err("Invalid file path".to_string())
            }
        }
        Ok(None) => Err("No file selected".to_string()),
        Err(_) => Err("Failed to receive file path".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_ollama,
            translate_text,
            open_ollama_website,
            open_file_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
