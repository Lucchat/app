pub mod commands;
pub mod keys;
#[macro_use]
pub mod logger;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // ajoute ici tes commandes, ex :
            commands::auth::login::login,
            commands::auth::register::register
        ])
        .setup(|app| {
            logger::init_logger(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
