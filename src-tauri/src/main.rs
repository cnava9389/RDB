#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use dotenv;

pub mod server;
pub mod models;
pub mod db;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    dotenv::dotenv().ok();
    let host = dotenv::var("VITE_SERVER_HOST").unwrap_or_default();
    let port = dotenv::var("VITE_SERVER_PORT").unwrap_or_default();
    std::thread::spawn(move || {
        if let Err(e) = server::start_server(&host, &port){
            // !todo log errors
            println!("Error starting server on {}:{} -> {}", host, port, e);
        };
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
