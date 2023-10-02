// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri::{Menu, Submenu};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let file_menu = Submenu::new("File", Menu::new());
    let edit_menu = Submenu::new("Edit", Menu::new());
    let view_menu = Submenu::new("View", Menu::new());
    let window_menu = Submenu::new("Window", Menu::new());
    let help_menu = Submenu::new("Help", Menu::new());
    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(window_menu)
        .add_submenu(help_menu);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
