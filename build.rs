use std::{ffi::OsString, fs::read_dir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<OsString> = read_dir("proto")
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap().ends_with(".proto")
        })
        .map(|entry| entry.file_name())
        .collect();
    tonic_build::configure().compile(&paths, &["proto"])?;
    Ok(())
}
