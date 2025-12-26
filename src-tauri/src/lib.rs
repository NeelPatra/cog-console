use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the main window
            let window = app.get_webview_window("main").unwrap();

            // Intercept navigation
            window.on_navigation(|url| {
                let url_str = url.as_str();
                // Allow navigation only if it's the game site
                if url_str.contains("cogdemos.ink") {
                    return true;
                }
                
                // Block all other links (social media, etc.)
                println!("Blocked navigation to: {}", url_str);
                false
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}