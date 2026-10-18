#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg_attr(feature = "cef", tauri_runtime_cef::cef_entry_point)]
fn main() {
    starter_tauri_v3_lib::run()
}
