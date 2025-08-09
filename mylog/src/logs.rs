//! This module implements three macros facilitating logging.

use std::fs::{self, DirEntry, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::env;
use std::time::{SystemTime, Duration};
use rayon::prelude::*;

use crate::utils::*;

const LOG_FILE_NAME: &str = "logs.txt";

/// Init the logs folder path by setting an environment variable called *_MYLOG_DIR_*
pub fn init(folder_path: String, file_size: String, duration: String) -> Result<(), String> {
    let folder_path = PathBuf::from(folder_path);

    if !fs::exists(&folder_path).unwrap_or(false) {
        fs::create_dir_all(&folder_path)
            .map_err(|e| format!(
                "Failed when try to create the folder : {}\n{}", folder_path.display(), e
            ))?;
    }

    let size_in_bytes = parse_file_size(&file_size)
        .ok_or(format!("Invalid file_size : {}", file_size))?;
    let duration_in_secs = parse_duration(&duration)
        .ok_or(format!("Invalid duration : {}", duration))?;

    unsafe {
        env::set_var("MYLOG_DIR", &folder_path);
    }

    let entries: Vec<DirEntry> = fs::read_dir(&folder_path)
        .map_err(|e| format!(
            "Failed when try to read the directory : {}\n{}", folder_path.display(), e
        ))?
        .flatten()
        .collect();

    let now = SystemTime::now();

    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        
        match path.extension().and_then(|e| e.to_str()).unwrap_or_default() {
            "txt" => {
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.len() >= size_in_bytes {
                        let _ = compress_to_zip(&path, &folder_path);
                    }
                }
            }
            "zip" => {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if now.duration_since(modified).unwrap_or(Duration::ZERO).as_secs() > duration_in_secs
                        {
                            let _ = fs::remove_file(path);
                        }
                    }
                }
            }
            _ => {}
        }
    });
    Ok(())
}

/// Get the path to the log file
pub fn get_log_path() -> PathBuf {
    let mylog_dir = env::var("MYLOG_DIR").unwrap_or(String::new());
    let mut path = PathBuf::from(mylog_dir);
    path.push(LOG_FILE_NAME);
    path
}

/// Write the content in argument in the __ at the root of the project.
pub fn write_log(content: String) {
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_log_path())
    {
        Ok(mut file) => {
            let _ = file.write_all(content.as_bytes());
        }
        Err(error) => {
            println!("Error when try to write log : {:?}", error);
        }
    }
}

#[macro_export]
/// Format the content in argument like the macro "format!()" <br>
/// and adding the current Timestamp and the level of to the log file.<br>
/// This has the following format : \[_TIMESTAMP_] \[INFO] \[_file_:_line_] \[_content_]
macro_rules! info {
    ($format_str:expr) => {{
        let msg = format!("[{}] [INFO] [{}:{}] : {}\n", $crate::utils::get_time(), file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[{}] [INFO] [{}:{}] : {}\n", $crate::utils::get_time(), file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}

#[macro_export]
/// Format the content in argument like the macro "format!()" <br>
/// and adding the current Timestamp and the level of to the log file.<br>
/// This has the following format : \[_TIMESTAMP_] \[WARNING] \[_file_:_line_] \[_content_]
macro_rules! warn {
    ($format_str:expr) => {{
        let msg = format!("[{}] [WARNING] [{}:{}] : {}\n", $crate::utils::get_time(), file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[{}] [WARNING] [{}:{}] : {}\n", $crate::utils::get_time(), file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}

#[macro_export]
/// Format the content in argument like the macro "format!()" <br>
/// and adding the current Timestamp and the level of to the log file.<br>
/// This has the following format : \[_TIMESTAMP_] \[ERROR] \[_file_:_line_] \[_content_]
macro_rules! error {
    ($format_str:expr) => {{
        let msg = format!("[{}] [ERROR] [{}:{}] : {}\n", $crate::utils::get_time(), file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[{}] [ERROR] [{}:{}] : {}\n", $crate::utils::get_time(), file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}
