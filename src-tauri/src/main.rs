// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use reqwest;
use regex::Regex;
use tauri::{Emitter, Manager};

#[derive(Debug, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaStreamResponse {
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
    think: bool,
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
        model: "qwen3:8b".to_string(),
        // 明确禁止输出思考标签或后缀
        prompt: format!(
            "请将以下英文文本翻译成中文，保持专业术语不变。\
             {}",
            text
        ),
        stream: false,
        think: false
    };

    let url = "http://127.0.0.1:11434/api/generate"; // 去掉尾部空格
    println!("[translate_text] 请求 URL: {}", url);

    match client.post(url).json(&request).send().await {
        Ok(resp) if resp.status().is_success() => {
            let ollama_resp: OllamaResponse = resp
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            // 去掉 <think>…</think> 以及尾巴上的 /no_think /think
            let re = Regex::new(r"(?s)<think>\s*.*?\s*</think>|\s*/no_think\s*/think\s*$").unwrap();
            let cleaned = re.replace_all(&ollama_resp.response, "").trim().to_string();

            println!("[translate_text] 翻译结果: {}", cleaned);
            Ok(cleaned)
        }
        Ok(resp) => {
            println!("[translate_text] ❌ 非 2xx 状态：{}", resp.status());
            Err("Translation failed".to_string())
        }
        Err(e) => {
            println!("[translate_text] ❌ 网络错误: {}", e);
            Err(format!("Network error: {}", e))
        }
    }
}

#[tauri::command]
async fn translate_text_stream(text: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("🌐 收到流式翻译请求，文本：{}", text);

    let client = reqwest::Client::new();
    let request = OllamaRequest {
        model: "qwen3:8b".to_string(),
        prompt: format!(
            "请将以下英文文本翻译成中文，保持专业术语不变。\
             {}",
            text
        ),
        stream: true,
        think: false,
    };

    let url = "http://127.0.0.1:11434/api/generate";
    println!("[translate_text_stream] 请求 URL: {}", url);

    match client.post(url).json(&request).send().await {
        Ok(mut resp) if resp.status().is_success() => {
            let mut full_response = String::new();
            
            while let Some(chunk) = resp.chunk().await.map_err(|e| format!("Failed to read chunk: {}", e))? {
                let chunk_str = String::from_utf8_lossy(&chunk);
                
                // 处理流式响应，每行是一个 JSON 对象
                for line in chunk_str.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    match serde_json::from_str::<OllamaStreamResponse>(line) {
                        Ok(stream_resp) => {
                            // 清理响应文本
                            let re = Regex::new(r"(?s)<think>\s*.*?\s*</think>|\s*/no_think\s*/think\s*$|\s*/think\s*$").unwrap();
                            let cleaned = re.replace_all(&stream_resp.response, "").trim().to_string();
                            
                            if !cleaned.is_empty() {
                                full_response.push_str(&cleaned);
                                
                                // 发送流式更新到前端 - 获取主窗口并发送事件
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.emit("translation_chunk", &cleaned);
                                    println!("[translate_text_stream] 发送流式数据: {}", cleaned);
                                } else {
                                    // 如果 "main" 窗口不存在，尝试获取第一个窗口
                                    if let Some(window) = app_handle.webview_windows().values().next() {
                                        let _ = window.emit("translation_chunk", &cleaned);
                                        println!("[translate_text_stream] 发送流式数据到第一个窗口: {}", cleaned);
                                    }
                                }
                            }
                            
                            if stream_resp.done {
                                println!("[translate_text_stream] 流式翻译完成");
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.emit("translation_complete", &full_response);
                                } else if let Some(window) = app_handle.webview_windows().values().next() {
                                    let _ = window.emit("translation_complete", &full_response);
                                }
                                println!("[translate_text_stream] 返回完整结果: {}", full_response);
                                return Ok(full_response);
                            }
                        }
                        Err(e) => {
                            println!("[translate_text_stream] JSON 解析错误: {}, 行: {}", e, line);
                        }
                    }
                }
            }
            
            Ok(full_response)
        }
        Ok(resp) => {
            println!("[translate_text_stream] ❌ 非 2xx 状态：{}", resp.status());
            Err("Translation failed".to_string())
        }
        Err(e) => {
            println!("[translate_text_stream] ❌ 网络错误: {}", e);
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
async fn get_pdf_data(path: String) -> Result<Vec<u8>, String> {
    use std::fs;
    
    println!("📁 Received request to read PDF: {}", path);
    
    // Remove the protocol prefix if present (Tauri 2.x uses asset://)
    let clean_path = path.replace("asset://localhost/", "");
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_ollama,
            translate_text,
            translate_text_stream,
            open_ollama_website,
            get_pdf_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
