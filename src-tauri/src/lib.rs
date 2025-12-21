use tauri::command;
use polars::prelude::*;
use std::fs::File;

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

#[command]
fn load_csv(path: String) -> String {
    // TRACE 1: Did Rust get the message?
    println!("DEBUG: 1. Rust received path: '{}'", path);

    let try_open = File::open(&path);

    match try_open {
        Ok(file) => {
            // TRACE 2: Did the file open?
            println!("DEBUG: 2. File opened! Starting Polars...");

            let reader = CsvReader::new(file);
            let result = reader.finish(); 

            match result {
                Ok(df) => {
                    // TRACE 3: Did Polars finish?
                    println!("DEBUG: 3. DataFrame Loaded! Rows: {}", df.height());
                    format!("CSV loaded successfully!\n\n{:?}", df)
                },
                Err(e) => {
                    // TRACE 4: Did Polars crash?
                    println!("DEBUG: ❌ Reader Crash: {}", e);
                    format!("ERROR reading CSV: {}", e)
                },
            }
        },
        Err(e) => {
            // TRACE 5: Did Windows block the file?
            println!("DEBUG: ❌ File Open Crash: {}", e);
            format!("ERROR opening file: {}", e)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![test_polars_connection, load_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}