#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[cfg(feature = "prod")]
const SERVER_URL: &str = "https://prodserver.com"; // TODO: Change this

#[cfg(not(feature = "prod"))]
const SERVER_URL: &str = "http://localhost:5000";

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = window.eval(&format!("window.location.replace('{}')", SERVER_URL));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
