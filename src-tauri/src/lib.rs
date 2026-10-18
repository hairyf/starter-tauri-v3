#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn run() {
    #[cfg(feature = "cef")]
    let builder = tauri::Builder::default().runtime(tauri_runtime_cef::Cef::default());

    #[cfg(not(feature = "cef"))]
    let builder = tauri::Builder::default().runtime(tauri_runtime_wry::Wry::default());

    builder
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_, _| {});
}
