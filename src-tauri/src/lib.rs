use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Creates another window through the runtime's `create_window` path; each
/// window is an independent Electron `BrowserWindow` with its own webview.
#[tauri::command]
fn open_second_window(app: tauri::AppHandle) -> Result<String, String> {
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(2);
    let label = format!(
        "window-{}",
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    );
    let handle = app.clone();
    tauri::WebviewWindowBuilder::new(&app, &label, tauri::WebviewUrl::App("index.html".into()))
        .title("starter-tauri-v3 — second window")
        .inner_size(720.0, 520.0)
        // `window.open()` from this window builds a Tauri window and renders the
        // requested page in it; the runtime answers Electron with that window.
        .on_new_window(move |url, _features| {
            static POPUP: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let label = format!(
                "popup-{}",
                POPUP.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            );
            match tauri::WebviewWindowBuilder::new(
                &handle,
                &label,
                tauri::WebviewUrl::External(url.clone()),
            )
            .title(format!("popup: {url}"))
            .inner_size(800.0, 600.0)
            .build()
            {
                Ok(window) => tauri::webview::NewWindowResponse::Create { window },
                Err(error) => {
                    eprintln!("[starter] failed to build popup window: {error}");
                    tauri::webview::NewWindowResponse::Deny
                }
            }
        })
        .build()
        .map(|_| label)
        .map_err(|error| error.to_string())
}

/// Ported from `deepseek-harness-desktop`: the tray icon owns the window
/// lifecycle, so hiding it keeps the app alive until "Quit" is picked.
fn install_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show window", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide window", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

    // Left click reveals the window, the menu stays on the right button - that is
    // what every other tray app does; a second click must not hide it again.
    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("starter-tauri-v3")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                // A click reports both edges; acting on both double-fires.
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(window) = tray.app_handle().get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        });

    // The Tauri-provided app icon doubles as the tray image; no extra asset.
    if let Some(icon) = app.default_window_icon().cloned() {
        builder = builder.icon(icon);
    }
    builder.build(app)?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .runtime(tauri_runtime_electron::Electron::default())
        .invoke_handler(tauri::generate_handler![greet, open_second_window])
        .setup(|app| {
            install_tray(app.handle())?;
            Ok(())
        })
        // Closing the window parks the app in the tray; quitting happens from the
        // tray menu, which is how `deepseek-harness-desktop` behaves by default.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_, _| {});
}
