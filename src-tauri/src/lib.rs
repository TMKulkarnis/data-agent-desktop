use tauri::command;
use polars::{prelude::*, sql::SQLContext};
use std::fs::File;
use std::iter::Scan;
use std::path::Path;

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
fn load_file(path: String) -> String {
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
                    println!("DEBUG: Reader Crash: {}", e);
                    format!("ERROR reading CSV: {}", e)
                },
            }
        },
        Err(e) => {
            // TRACE 5: Did Windows block the file?
            println!("DEBUG:  File Open Crash: {}", e);
            format!("ERROR opening file: {}", e)
        }
    }
}

#[command]
fn query_table(path: String, query: String) -> String {
    let mut ctx = SQLContext::new();
    let p = Path::new(&path);

    let extension = p.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("csv");

let lazy_frame_result = match extension {
        "csv" => {
            LazyCsvReader::new(PlPath::new(&path))
                .with_has_header(true)
                .finish()
        },
        "json" | "jsonl" => {
            LazyJsonLineReader::new(PlPath::new(&path))
                .finish()
        },
        "parquet" => {
            LazyFrame::scan_parquet(PlPath::new(&path), ScanArgsParquet::default())
        },
        "tsv" => {
            LazyCsvReader::new(PlPath::new(&path))
                .with_separator(b'\t') 
                .with_has_header(true)
                .finish()
        },
        _ => {// Default to CSV if unknown
            LazyCsvReader::new(PlPath::new(&path))
                .with_has_header(true)
                .finish()
        }
    };

    match lazy_frame_result {
        Ok(lf) => {
            ctx.register("data", lf);
            
            match ctx.execute(&query) {
                Ok(lazy_result) => {
                    match lazy_result.collect() {
                        Ok(df) => format!("Query Results [Format: {}]:\n\n{:?}", extension.to_uppercase(), df),
                        Err(e) => format!("SQL Execution Error: {}", e),
                    }
                },
                Err(e) => format!("SQL Parsing Error: {}", e),
            }
        },
        Err(e) => format!("Error opening file: {}", e),
    }
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![test_polars_connection, load_file,query_table])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}