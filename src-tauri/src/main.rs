// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command ,Child};
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    static ref CLIENT: Mutex<Option<String>> = Mutex::new(None);
}

fn set_global_var(value: String) {
    let mut global_var = CLIENT.lock().unwrap();
    *global_var = Some(value);
}

fn get_global_var() -> Option<String> {
    let global_var = CLIENT.lock().unwrap();
    global_var.clone()  // Return a clone of the Option<String>
}


#[derive(Deserialize)]
struct Config {
    model: String,
    server: String,
    client: String,
}

#[tauri::command]
fn start_llama_server() -> Result<Child, String> {

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let project_root = current_dir.parent().expect("Failed to get project root");

    let config_file = project_root.join("model_config.json");
    let mut file = File::open(config_file).expect("Failed to open model_config.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read model_config.json");

    let config: Config = serde_json::from_str(&contents).expect("Failed to parse model_config.json");

    let model_file = config.model;
    let model_path = project_root.join("bin").join(model_file);

    let server_file = config.server;
    let server_path = project_root.join("bin").join(server_file);

    let client_file = config.client;
    let client_path = project_root.join("bin").join(client_file);

    set_global_var(client_path.to_str().unwrap().to_string());

    let child = Command::new(server_path)
        .arg("--log-disable")
        .arg("-m")
        .arg(model_path)
        .arg("-ngl")
        .arg("43")
        .spawn()
        .map_err(|e| format!("Failed to start llama-server: {}", e))?;

    println!("Started llama server");

    Ok(child)
}

#[tauri::command]
fn translate_jp_to_en(text: String) -> Result<String, String> {
    let client = get_global_var().unwrap();
    let output = Command::new(client)
        .arg(&text)
        .output()
        .expect("failed to execute process");

    let translation = String::from_utf8(output.stdout).unwrap_or_else(|_| "Error in translation".to_string());
    Ok(translation)
}

#[tauri::command]
fn translate_en_to_jp(text: String) -> Result<String, String> {
    let client = get_global_var().unwrap();
    let output = Command::new(client)
        .arg(&text)
        .output()
        .expect("failed to execute process");

    let translation = String::from_utf8(output.stdout).unwrap_or_else(|_| "Error in translation".to_string());
    Ok(translation)
}

fn main() {
    let _ = start_llama_server();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![translate_jp_to_en, translate_en_to_jp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
