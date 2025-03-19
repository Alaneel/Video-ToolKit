use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write, Seek, SeekFrom};

use walkdir::WalkDir;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

use common::{Result, VideoToolkitError};

/// Checks if a file is a GIF by verifying its magic number
fn is_gif_file(path: &Path) -> bool {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; 6];
        if file.read_exact(&mut buffer).is_ok() {
            // Check for the GIF magic number (GIF87a or GIF89a)
            return buffer.starts_with(b"GIF87a") || buffer.starts_with(b"GIF89a");
        }
    }
    false
}

/// Converts the final byte of a GIF file from 0x3B to 0x21 to create transparency
pub fn make_gif_transparent(file_path: &Path) -> Result<()> {
    // Open the file for reading and writing
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .map_err(|e| VideoToolkitError::IoError(e))?;

    // Verify it's a GIF file
    let mut header = [0; 6];
    file.read_exact(&mut header)
        .map_err(|e| VideoToolkitError::IoError(e))?;

    if !header.starts_with(b"GIF87a") && !header.starts_with(b"GIF89a") {
        return Err(VideoToolkitError::Other(format!(
            "Not a valid GIF file: {}",
            file_path.display()
        )));
    }

    // Get the file size
    let file_size = file
        .seek(SeekFrom::End(0))
        .map_err(|e| VideoToolkitError::IoError(e))?;

    if file_size < 1 {
        return Err(VideoToolkitError::Other("GIF file is too small".to_string()));
    }

    // Read the last byte
    let last_position = file_size - 1;
    file.seek(SeekFrom::Start(last_position))
        .map_err(|e| VideoToolkitError::IoError(e))?;

    let mut last_byte = [0; 1];
    file.read_exact(&mut last_byte)
        .map_err(|e| VideoToolkitError::IoError(e))?;

    // Check if the last byte is 0x3B (GIF trailer)
    if last_byte[0] == 0x3B {
        // Move back to the last byte position
        file.seek(SeekFrom::Start(last_position))
            .map_err(|e| VideoToolkitError::IoError(e))?;

        // Replace it with 0x21 (GIF extension introducer)
        file.write_all(&[0x21])
            .map_err(|e| VideoToolkitError::IoError(e))?;

        return Ok(());
    } else if last_byte[0] == 0x21 {
        // Already transparent
        return Ok(());
    } else {
        return Err(VideoToolkitError::Other(format!(
            "Unexpected GIF trailer byte: 0x{:02X}",
            last_byte[0]
        )));
    }
}

/// Process multiple GIF files in batch, making them transparent
pub fn batch_process_gifs(
    input_paths: &[PathBuf],
    recursive: bool,
    create_backup: bool,
) -> Result<(usize, usize)> {
    // Collect all GIF files
    let mut gif_files = Vec::new();

    for path in input_paths {
        if path.is_dir() && recursive {
            // Recursively walk directory
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                if entry_path.is_file() &&
                    entry_path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("gif")) &&
                    is_gif_file(entry_path) {
                    gif_files.push(entry_path.to_owned());
                }
            }
        } else if path.is_file() &&
            path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("gif")) &&
            is_gif_file(path) {
            gif_files.push(path.to_owned());
        }
    }

    if gif_files.is_empty() {
        return Err(VideoToolkitError::Other("No GIF files found".to_string()));
    }

    let total_files = gif_files.len();
    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
    );

    // Create backups if requested
    if create_backup {
        for file_path in &gif_files {
            let backup_path = file_path.with_extension("gif.bak");
            fs::copy(file_path, backup_path)
                .map_err(|e| VideoToolkitError::IoError(e))?;
        }
    }

    // Process files in parallel
    let results: Vec<Result<()>> = gif_files
        .par_iter()
        .map(|file_path| {
            let result = make_gif_transparent(file_path);
            progress_bar.inc(1);
            result
        })
        .collect();

    progress_bar.finish_with_message("GIF processing complete");

    // Count successful operations
    let success_count = results.iter().filter(|r| r.is_ok()).count();

    Ok((success_count, total_files))
}

/// Find and process all GIFs in a directory
pub fn process_directory(
    dir_path: &str,
    recursive: bool,
    create_backup: bool,
) -> Result<(usize, usize)> {
    let path = Path::new(dir_path);

    if !path.exists() {
        return Err(VideoToolkitError::Other(format!("Directory not found: {}", dir_path)));
    }

    if !path.is_dir() {
        return Err(VideoToolkitError::Other(format!("Not a directory: {}", dir_path)));
    }

    batch_process_gifs(&[path.to_path_buf()], recursive, create_backup)
}