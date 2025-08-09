use chrono::Local;
use std::path::PathBuf;
use zip::write::{FileOptions, SimpleFileOptions};
use std::fs::File;
use std::io::{Write, Read};

/// Return the current Timestamp.
pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn compress_to_zip(file_path: &PathBuf, folder_path: &PathBuf) -> std::io::Result<()> {
    let zip_name = folder_path.join(format!("{}.zip", get_time()));
    let zip_file = File::create(&zip_name)?;
    let mut zip = zip::ZipWriter::new(zip_file);

    let options: SimpleFileOptions = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    if let Some(fname) = file_path.file_name() {
        zip.start_file(fname.to_string_lossy().into_owned(), options)?;
    }

    let mut f = File::open(file_path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    zip.finish()?;

    std::fs::remove_file(file_path)?;
    Ok(())
}


pub fn parse_file_size(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim().to_lowercase();
    let num_part= size_str.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let unit_part = size_str.chars()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let number = num_part.parse::<u64>().ok()?;

    let multiplier = match unit_part.trim() {
        "b" => 1,
        "kb" | "ko" => 1_000,
        "mb" | "mo" => 1_000_000,
        "gb" | "go" => 1_000_000_000,
        "kib" => 1 << 10,
        "mib" => 1 << 20,
        "gib" => 1 << 30,
        _ => return None,
    };

    Some(number * multiplier)
}

pub fn parse_duration(duration_str: &str) -> Option<u64> {
    let duration_str = duration_str.trim().to_lowercase();
    let num_part = duration_str.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let unit_part = duration_str.chars()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let number: u64 = num_part.parse().ok()?;

    let multiplier = match unit_part.trim() {
        "s" | "sec" | "secs" | "seconds" => 1,
        "m" | "min" | "mins" | "minutes" => 60,
        "h" | "hr" | "hrs" | "hours" => 3600,
        "d" | "day" | "days" => 86400,
        _ => return None,
    };

    Some(number * multiplier)
}
