#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod menu;

fn main() {
    let builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    let builder = builder.menu(menu::menu());

    let builder = builder.setup(|app| {
        if let Ok(matches) = app.get_cli_matches() {
            for (k, v) in matches.args {
                #[allow(clippy::single_match)]
                match k.as_str() {
                    "proxy-server" => {},
                    _ => {},
                }
            }
        };
        Ok(())
    });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
