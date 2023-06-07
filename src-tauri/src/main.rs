// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;

fn main() {
    tauri::Builder::default()
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
              event.window().hide().unwrap();
              api.prevent_close();
            }
            _ => {}
     })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}