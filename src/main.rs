// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let width = 800.;
            let height = 600.;

            let window = tauri::WindowBuilder::new(app, "main_window")
                .inner_size(width, height)
                .build()?;

            // let _main_webview = window.add_child(
            //     tauri::webview::WebviewBuilder::new("main_webview", tauri::WebviewUrl::App(Default::default()))
            //         .auto_resize(),
            //     tauri::LogicalPosition::new(0., 0.),
            //     tauri::LogicalSize::new(width / 2., height / 2.),
            // )?;

            let _google_docs_webview = window.add_child(
                tauri::WebviewBuilder::new(
                    "google_docs_webview",
                    tauri::WebviewUrl::External("https://docs.google.com/document/u/0/create".parse().unwrap()),
                )
                    .auto_resize(),
                tauri::LogicalPosition::new(0., 0.),
                tauri::LogicalSize::new(width, height),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
