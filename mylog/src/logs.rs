use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE_PATH: &str = "logs.txt";

#[allow(unused)]
pub fn write_log(content: String) {
    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_FILE_PATH)
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
macro_rules! info {
    ($format_str:expr) => {{
        let msg = format!("[INFO] [{}:{}] : {}\n", file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[INFO] [{}:{}] : {}\n", file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}

#[macro_export]
macro_rules! warn {
    ($format_str:expr) => {{
        let msg = format!("[WARNING] [{}:{}] : {}\n", file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[WARNING] [{}:{}] : {}\n", file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}

#[macro_export]
macro_rules! error {
    ($format_str:expr) => {{
        let msg = format!("[ERROR] [{}:{}] : {}\n", file!(), line!(), $format_str);
        $crate::logs::write_log(msg);
    }};
    ($format_str:expr, $($arg:tt)*) => {{
        let msg = format!("[ERROR] [{}:{}] : {}\n", file!(), line!(), format!($format_str, $($arg)*));
        $crate::logs::write_log(msg);
    }};
}
