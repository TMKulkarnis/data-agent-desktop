use tauri::command;
use polars::prelude::*;

#[command]
fn test_polars_connection() -> String {
    let df = df! (
        "names" => &["Data Agent", "Rust", "Tauri"],
        "values" => &[1, 10, 100],
        "is_fast" => &[true, true, true]
    );

    match df {
        Ok(data) => format!("Polars is running!\n\n{:?}", data),
        Err(e) => format!("ERROR: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_polars_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}