use std::path::Path;
use crate::formats::{FormatType, detect_format};
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

/// Get output container format extension based on input format and conversion type
pub fn get_output_format(input_path: &Path, target_format: Option<&str>) -> Result<String> {
    // If target format is explicitly specified, use that
    if let Some(format) = target_format {
        return Ok(format.to_string());
    }

    // Otherwise, infer from input file
    if let Some(format_type) = detect_format(input_path) {
        match format_type {
            FormatType::Video(_) => Ok("mp4".to_string()),
            FormatType::Audio(_) => Ok("mp3".to_string()),
            FormatType::Image(_) => Ok("png".to_string()),
        }
    } else {
        // Default to MP4 if unable to determine
        Ok("mp4".to_string())
    }
}

/// Get FFmpeg codec options for a specific format
pub fn get_codec_options(format: &str) -> Vec<String> {
    match format.to_lowercase().as_str() {
        // Video formats
        "mp4" => vec!["-c:v", "libx264", "-c:a", "aac"],
        "webm" => vec!["-c:v", "libvpx", "-c:a", "libvorbis"],
        "mkv" => vec!["-c:v", "libx264", "-c:a", "aac"],
        "avi" => vec!["-c:v", "libx264", "-c:a", "mp3"],
        "mov" => vec!["-c:v", "libx264", "-c:a", "aac"],
        "flv" => vec!["-c:v", "libx264", "-c:a", "aac"],
        "wmv" => vec!["-c:v", "wmv2", "-c:a", "wmav2"],
        "ogv" => vec!["-c:v", "libtheora", "-c:a", "libvorbis"],

        // Audio formats
        "mp3" => vec!["-c:a", "libmp3lame"],
        "aac" => vec!["-c:a", "aac"],
        "wav" => vec!["-c:a", "pcm_s16le"],
        "flac" => vec!["-c:a", "flac"],
        "ogg" => vec!["-c:a", "libvorbis"],
        "m4a" => vec!["-c:a", "aac"],

        // Image formats (animation)
        "gif" => vec!["-c:v", "gif"],
        "apng" => vec!["-c:v", "apng"],

        // Default to H.264 + AAC
        _ => vec!["-c:v", "libx264", "-c:a", "aac"],
    }.iter().map(|s| s.to_string()).collect()
}

/// Check if a format is supported for a specific operation
pub fn is_format_supported_for_operation(format: &str, operation: &str) -> bool {
    match operation {
        "clipper" => {
            matches!(format.to_lowercase().as_str(),
                "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" | "ts" | "m4v" | "mpeg" | "ogv")
        },
        "gif_converter" => {
            matches!(format.to_lowercase().as_str(),
                "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv")
        },
        "gif_transparency" => {
            format.to_lowercase() == "gif"
        },
        "splitter" => {
            matches!(format.to_lowercase().as_str(),
                "mp4" | "mkv" | "avi" | "mov" | "webm")
        },
        "merger" => {
            // Audio formats for the audio component
            matches!(format.to_lowercase().as_str(),
                "mp3" | "aac" | "wav" | "flac" | "ogg" | "m4a") ||
                // Video formats for the video component
                matches!(format.to_lowercase().as_str(),
                "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv")
        },
        _ => false,
    }
}

/// Get all supported formats for a specific operation
pub fn get_supported_formats(operation: &str) -> Vec<String> {
    match operation {
        "clipper" => {
            vec!["mp4", "mkv", "avi", "mov", "webm", "flv", "ts", "m4v", "mpeg", "ogv"]
                .iter().map(|s| s.to_string()).collect()
        },
        "gif_converter" => {
            vec!["mp4", "mkv", "avi", "mov", "webm", "flv"]
                .iter().map(|s| s.to_string()).collect()
        },
        "gif_transparency" => {
            vec!["gif"].iter().map(|s| s.to_string()).collect()
        },
        "splitter" => {
            vec!["mp4", "mkv", "avi", "mov", "webm"]
                .iter().map(|s| s.to_string()).collect()
        },
        "merger" => {
            // Audio formats
            let audio = vec!["mp3", "aac", "wav", "flac", "ogg", "m4a"];
            // Video formats
            let video = vec!["mp4", "mkv", "avi", "mov", "webm", "flv"];

            [audio, video].concat().iter().map(|s| s.to_string()).collect()
        },
        _ => Vec::new(),
    }
}