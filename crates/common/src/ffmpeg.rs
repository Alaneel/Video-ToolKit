use std::path::Path;
use std::process::{Command, Output};
use regex::Regex;
use lazy_static::lazy_static;

use crate::error::{Result, VideoToolkitError};

/// Check if FFmpeg is installed and accessible
pub fn check_ffmpeg() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|_| true)
        .unwrap_or(false)
}

/// Execute an FFmpeg command with the given arguments
pub fn execute_ffmpeg(args: &[&str]) -> Result<Output> {
    let output = Command::new("ffmpeg")
        .args(args)
        .output()
        .map_err(|e| VideoToolkitError::IoError(e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(VideoToolkitError::FFmpegCommandFailed(error_message.to_string()));
    }

    Ok(output)
}

/// Get video dimensions using FFprobe
pub fn get_video_dimensions(file_path: &str) -> Result<(u32, u32)> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height",
            "-of", "csv=p=0",
            file_path
        ])
        .output()
        .map_err(|e| VideoToolkitError::IoError(e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(VideoToolkitError::FFmpegCommandFailed(error_message.to_string()));
    }

    let dimensions = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = dimensions.trim().split(',').collect();

    if parts.len() != 2 {
        return Err(VideoToolkitError::DimensionsError);
    }

    let width = parts[0].parse::<u32>()
        .map_err(|_| VideoToolkitError::DimensionsError)?;
    let height = parts[1].parse::<u32>()
        .map_err(|_| VideoToolkitError::DimensionsError)?;

    Ok((width, height))
}

/// Verify input file exists
pub fn verify_input_file(file_path: &str) -> Result<()> {
    if !Path::new(file_path).exists() {
        return Err(VideoToolkitError::InputFileNotFound(file_path.to_string()));
    }
    Ok(())
}

// Timestamp validation patterns
lazy_static! {
    pub static ref TIMESTAMP_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"^\d+$").unwrap(),                      // Seconds only
        Regex::new(r"^\d+\.\d+$").unwrap(),                 // Seconds with decimal
        Regex::new(r"^\d+:\d{2}$").unwrap(),                // MM:SS
        Regex::new(r"^\d+:\d{2}\.\d+$").unwrap(),           // MM:SS.mmm
        Regex::new(r"^\d+:\d{2}:\d{2}$").unwrap(),          // HH:MM:SS
        Regex::new(r"^\d+:\d{2}:\d{2}\.\d+$").unwrap(),     // HH:MM:SS.mmm
    ];
}

/// Validate timestamp format (HH:MM:SS or MM:SS or SS or HH:MM:SS.mmm)
pub fn validate_timestamp(timestamp: &str) -> bool {
    TIMESTAMP_PATTERNS.iter().any(|pattern| pattern.is_match(timestamp))
}

/// Validate time range format (start-end)
pub fn validate_time_range(time_range: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = time_range.split('-').collect();
    if parts.len() != 2 {
        return None;
    }

    let start_time = parts[0].trim();
    let end_time = parts[1].trim();

    if !validate_timestamp(start_time) || !validate_timestamp(end_time) {
        return None;
    }

    Some((start_time.to_string(), end_time.to_string()))
}

/// Get file size in megabytes
pub fn get_file_size_mb(file_path: &Path) -> f64 {
    match std::fs::metadata(file_path) {
        Ok(metadata) => metadata.len() as f64 / (1024.0 * 1024.0),
        Err(_) => 0.0,
    }
}