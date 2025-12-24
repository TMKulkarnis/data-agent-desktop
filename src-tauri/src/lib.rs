use tauri::command;
use polars::{prelude::*, sql::SQLContext};
use std::fs::File;
use std::path::Path;
use calamine::{Reader, Xlsx, open_workbook};
use reqwest::blocking::get;
use std::env;
use uuid::Uuid;

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
                .with_infer_schema_length(None)
                .finish()
        },
        "json" | "jsonl" => {
            LazyJsonLineReader::new(PlPath::new(&path))
                .with_infer_schema_length(None)
                .finish()
        },
        "parquet" => {
            LazyFrame::scan_parquet(PlPath::new(&path), ScanArgsParquet::default())
        },
        "tsv" => {
            LazyCsvReader::new(PlPath::new(&path))
                .with_separator(b'\t') 
                .with_has_header(true)
                .with_infer_schema_length(None)
                .finish()
        },
        "xlsx" => {
            let mut workbook: Xlsx<_> = open_workbook(&path).expect("Cannot open Excel file");
            if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                let (rows, cols) = range.get_size();

                let df_result = df!{
                    "status" => &["Excel file loaded successfully!"],
                    "rows" => &[rows as u32],
                    "cols" => &[cols as u32]
                };
                match df_result {
                    Ok(df) => Ok(df.lazy()), 
                    Err(e) => Err(e),
                }
            } 
            else {
                Err(PolarsError::ComputeError("Could not read the first sheet of the Excel file.".into()))
            }
        },

        _ => {// Default to CSV if unknown
            LazyCsvReader::new(PlPath::new(&path))
                .with_has_header(true)
                .with_infer_schema_length(None)
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

#[command]
fn load_url(url: String) -> String{
    let mut temp_path = env::temp_dir();
    let filename = format!("data_agent_{}.tmp", Uuid::new_v4());
    temp_path.push(filename);
    let path_str = temp_path.to_str().unwrap().to_string();

   match get(&url) {
        Ok(response) => {
            if response.status().is_success() {
                let save_result = (|| -> std::io::Result<()> {
                    let mut dest = File::create(&path_str)?;
                    let content = response.bytes().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                    let mut content_reader = std::io::Cursor::new(content);
                    std::io::copy(&mut content_reader, &mut dest)?;
                    Ok(())
                })();

                match save_result {
                    Ok(_) => path_str, 
                    Err(e) => format!("ERROR: Disk Write Failed: {}", e),
                }
            } else {
                format!("ERROR: Server returned status: {}", response.status())
            }
        },
        Err(e) => format!("ERROR: Connection Failed: {}", e),
    }

}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![test_polars_connection, load_file,query_table,load_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}