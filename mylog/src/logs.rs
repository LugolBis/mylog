//! This module implements three macros facilitating logging.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CStr;
use std::env;
use libc::{localtime, strftime, time_t, tm};

const LOG_FILE_NAME: &str = "logs.txt";

/// Init the logs folder path by setting an environment variable called *_MYLOG_DIR_*
pub fn init(folder_path: String) {
    if !fs::exists(&folder_path).unwrap_or(false) {
        let _ = fs::create_dir_all(&folder_path);
    }

    unsafe {
        env::set_var("MYLOG_DIR", folder_path);
    }
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

/// Return the current Timestamp.
pub fn get_time() -> String {
    let system_time = SystemTime::now();
    let duration = system_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration.as_secs() as time_t;

    unsafe {
        let tm_ptr: *mut tm = localtime(&timestamp);
        let mut buffer = [0u8; 100];

        let format = "%Y-%m-%d %H:%M:%S\0";
        let len = strftime(
            buffer.as_mut_ptr() as *mut _,
            buffer.len(),
            format.as_ptr() as *const _,
            tm_ptr,
        );

        if len > 0 {
            let cstr = CStr::from_ptr(buffer.as_ptr() as *const _);
            String::from(cstr.to_str().unwrap_or("~Date~"))
        } else {
            String::from("~Date~")
        }
    }
}


#[macro_export]
/// Format the content in argument like the macro "format!()" <br>
/// and adding the current Timestamp and the level of to the log file.<br>
/// This has the following format : \[_TIMESTAMP_] \[INFO] \[_file_:_line_] \[_content_]
macro_rules! info {
    ($format_str:expr) => {{
        let msg = format!("[{}] [INFO] [{}:{}] : {}\n", $crate::logs::get_time(), file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[{}] [INFO] [{}:{}] : {}\n", $crate::logs::get_time(), file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}

#[macro_export]
/// Format the content in argument like the macro "format!()" <br>
/// and adding the current Timestamp and the level of to the log file.<br>
/// This has the following format : \[_TIMESTAMP_] \[WARNING] \[_file_:_line_] \[_content_]
macro_rules! warn {
    ($format_str:expr) => {{
        let msg = format!("[{}] [WARNING] [{}:{}] : {}\n", $crate::logs::get_time(), file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[{}] [WARNING] [{}:{}] : {}\n", $crate::logs::get_time(), file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}

#[macro_export]
/// Format the content in argument like the macro "format!()" <br>
/// and adding the current Timestamp and the level of to the log file.<br>
/// This has the following format : \[_TIMESTAMP_] \[ERROR] \[_file_:_line_] \[_content_]
macro_rules! error {
    ($format_str:expr) => {{
        let msg = format!("[{}] [ERROR] [{}:{}] : {}\n", $crate::logs::get_time(), file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[{}] [ERROR] [{}:{}] : {}\n", $crate::logs::get_time(), file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}
