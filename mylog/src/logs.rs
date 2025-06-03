use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ffi::CStr;
use libc::{localtime, strftime, time_t, tm};

const LOG_FILE_PATH: &str = "logs.txt";

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
