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

#[derive(Debug, Serialize, Deserialize)]
struct ModelConfig {
    selected_model: String,
    ollama: OllamaConfig,
    deepseek: DeepSeekConfig,
    kimi: KimiConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaConfig {
    base_url: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekConfig {
    api_key: String,
    base_url: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KimiConfig {
    api_key: String,
    base_url: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct KimiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KimiRequest {
    model: String,
    messages: Vec<KimiMessage>,
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

#[tauri::command]
async fn translate_with_config(text: String, config: ModelConfig, app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("🌐 收到多模型翻译请求，文本：{}，模型：{}", text, config.selected_model);
    
    match config.selected_model.as_str() {
        "ollama" => {
            println!("使用 Ollama 模型进行翻译");
            translate_with_ollama(text, config.ollama, app_handle).await
        }
        "deepseek" => {
            println!("使用 DeepSeek 模型进行翻译");
            translate_with_deepseek(text, config.deepseek, app_handle).await
        }
        "kimi" => {
            println!("使用 Kimi 模型进行翻译");
            translate_with_kimi(text, config.kimi, app_handle).await
        }
        _ => {
            println!("❌ 未知模型类型：{}", config.selected_model);
            Err("Unknown model type".to_string())
        }
    }
}

async fn translate_with_ollama(text: String, config: OllamaConfig, app_handle: tauri::AppHandle) -> Result<String, String> {
    let client = reqwest::Client::new();
    let request = OllamaRequest {
        model: config.model,
        prompt: format!(
            "请将以下英文文本翻译成中文，保持专业术语不变。\
             {}",
            text
        ),
        stream: true,
        think: false,
    };

    let url = format!("{}/api/generate", config.base_url);
    println!("[translate_with_ollama] 请求 URL: {}", url);

    match client.post(&url).json(&request).send().await {
        Ok(mut resp) if resp.status().is_success() => {
            let mut full_response = String::new();
            
            while let Some(chunk) = resp.chunk().await.map_err(|e| format!("Failed to read chunk: {}", e))? {
                let chunk_str = String::from_utf8_lossy(&chunk);
                
                for line in chunk_str.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    match serde_json::from_str::<OllamaStreamResponse>(line) {
                        Ok(stream_resp) => {
                            let re = Regex::new(r"(?s)<think>\s*.*?\s*</think>|\s*/no_think\s*/think\s*$|\s*/think\s*$").unwrap();
                            let cleaned = re.replace_all(&stream_resp.response, "").trim().to_string();
                            
                            if !cleaned.is_empty() {
                                full_response.push_str(&cleaned);
                                
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.emit("translation_chunk", &cleaned);
                                } else if let Some(window) = app_handle.webview_windows().values().next() {
                                    let _ = window.emit("translation_chunk", &cleaned);
                                }
                            }
                            
                            if stream_resp.done {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.emit("translation_complete", &full_response);
                                } else if let Some(window) = app_handle.webview_windows().values().next() {
                                    let _ = window.emit("translation_complete", &full_response);
                                }
                                return Ok(full_response);
                            }
                        }
                        Err(e) => {
                            println!("[translate_with_ollama] JSON 解析错误: {}, 行: {}", e, line);
                        }
                    }
                }
            }
            
            Ok(full_response)
        }
        Ok(resp) => {
            println!("[translate_with_ollama] ❌ 非 2xx 状态：{}", resp.status());
            Err("Ollama translation failed".to_string())
        }
        Err(e) => {
            println!("[translate_with_ollama] ❌ 网络错误: {}", e);
            Err(format!("Network error: {}", e))
        }
    }
}

async fn translate_with_deepseek(text: String, config: DeepSeekConfig, app_handle: tauri::AppHandle) -> Result<String, String> {
    if config.api_key.is_empty() {
        return Err("DeepSeek API key is required".to_string());
    }

    let client = reqwest::Client::new();
    let request = DeepSeekRequest {
        model: config.model,
        messages: vec![
            DeepSeekMessage {
                role: "system".to_string(),
                content: "你是一个专业的翻译助手，请将英文文本准确翻译成中文，保持专业术语不变。".to_string(),
            },
            DeepSeekMessage {
                role: "user".to_string(),
                content: text,
            },
        ],
        stream: true,
    };

    let url = format!("{}/chat/completions", config.base_url);
    println!("[translate_with_deepseek] 请求 URL: {}", url);

    match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(mut resp) if resp.status().is_success() => {
            let mut full_response = String::new();
            
            while let Some(chunk) = resp.chunk().await.map_err(|e| format!("Failed to read chunk: {}", e))? {
                let chunk_str = String::from_utf8_lossy(&chunk);
                
                for line in chunk_str.lines() {
                    if line.trim().is_empty() || !line.starts_with("data: ") {
                        continue;
                    }
                    
                    let data_line = &line[6..]; // Remove "data: " prefix
                    if data_line.trim() == "[DONE]" {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.emit("translation_complete", &full_response);
                        } else if let Some(window) = app_handle.webview_windows().values().next() {
                            let _ = window.emit("translation_complete", &full_response);
                        }
                        return Ok(full_response);
                    }
                    
                    match serde_json::from_str::<serde_json::Value>(data_line) {
                        Ok(json) => {
                            if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                                if let Some(choice) = choices.first() {
                                    if let Some(delta) = choice.get("delta") {
                                        if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                            if !content.is_empty() {
                                                full_response.push_str(content);
                                                
                                                if let Some(window) = app_handle.get_webview_window("main") {
                                                    let _ = window.emit("translation_chunk", content);
                                                } else if let Some(window) = app_handle.webview_windows().values().next() {
                                                    let _ = window.emit("translation_chunk", content);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("[translate_with_deepseek] JSON 解析错误: {}, 行: {}", e, data_line);
                        }
                    }
                }
            }
            
            Ok(full_response)
        }
        Ok(resp) => {
            println!("[translate_with_deepseek] ❌ 非 2xx 状态：{}", resp.status());
            Err("DeepSeek translation failed".to_string())
        }
        Err(e) => {
            println!("[translate_with_deepseek] ❌ 网络错误: {}", e);
            Err(format!("Network error: {}", e))
        }
    }
}

async fn translate_with_kimi(text: String, config: KimiConfig, app_handle: tauri::AppHandle) -> Result<String, String> {
    if config.api_key.is_empty() {
        return Err("Kimi API key is required".to_string());
    }

    let client = reqwest::Client::new();
    let request = KimiRequest {
        model: config.model,
        messages: vec![
            KimiMessage {
                role: "system".to_string(),
                content: "你是一个专业的翻译助手，请将英文文本准确翻译成中文，保持专业术语不变。".to_string(),
            },
            KimiMessage {
                role: "user".to_string(),
                content: text,
            },
        ],
        stream: true,
    };

    let url = format!("{}/v1/chat/completions", config.base_url);
    println!("[translate_with_kimi] 请求 URL: {}", url);

    match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(mut resp) if resp.status().is_success() => {
            let mut full_response = String::new();
            
            while let Some(chunk) = resp.chunk().await.map_err(|e| format!("Failed to read chunk: {}", e))? {
                let chunk_str = String::from_utf8_lossy(&chunk);
                
                for line in chunk_str.lines() {
                    if line.trim().is_empty() || !line.starts_with("data: ") {
                        continue;
                    }
                    
                    let data_line = &line[6..]; // Remove "data: " prefix
                    if data_line.trim() == "[DONE]" {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.emit("translation_complete", &full_response);
                        } else if let Some(window) = app_handle.webview_windows().values().next() {
                            let _ = window.emit("translation_complete", &full_response);
                        }
                        return Ok(full_response);
                    }
                    
                    match serde_json::from_str::<serde_json::Value>(data_line) {
                        Ok(json) => {
                            if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
                                if let Some(choice) = choices.first() {
                                    if let Some(delta) = choice.get("delta") {
                                        if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                            if !content.is_empty() {
                                                full_response.push_str(content);
                                                
                                                if let Some(window) = app_handle.get_webview_window("main") {
                                                    let _ = window.emit("translation_chunk", content);
                                                } else if let Some(window) = app_handle.webview_windows().values().next() {
                                                    let _ = window.emit("translation_chunk", content);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("[translate_with_kimi] JSON 解析错误: {}, 行: {}", e, data_line);
                        }
                    }
                }
            }
            
            Ok(full_response)
        }
        Ok(resp) => {
            println!("[translate_with_kimi] ❌ 非 2xx 状态：{}", resp.status());
            Err("Kimi translation failed".to_string())
        }
        Err(e) => {
            println!("[translate_with_kimi] ❌ 网络错误: {}", e);
            Err(format!("Network error: {}", e))
        }
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
            get_pdf_data,
            translate_with_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
