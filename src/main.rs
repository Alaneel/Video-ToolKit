use eframe::NativeOptions;
use clap::{Parser, Subcommand};

use common::check_ffmpeg;
use clipper::{clip_video, parse_time_ranges};
use gif_converter::{convert_mp4_to_gif, optimize_conversion};
use splitter::split_video;
use merger::merge_audio_video;
use ui::VideoToolKitApp;

#[derive(Parser)]
#[clap(author, version, about = "Video processing utilities")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract specific time ranges from MP4 video files
    Clipper {
        /// Input MP4 file path
        input: String,

        /// Time ranges to extract in format START-END (e.g., 00:01:00-00:02:00)
        #[clap(short, long, required = true)]
        ranges: Vec<String>,

        /// Output directory for video clips
        #[clap(short, long, default_value = "output_clips")]
        output_dir: String,

        /// Copy codec instead of re-encoding (faster but may be less precise)
        #[clap(long)]
        copy_codec: bool,

        /// Optional suffix to add to output filenames
        #[clap(short, long)]
        suffix: Option<String>,
    },

    /// Convert MP4 videos to optimized GIF format
    GifConverter {
        /// Input MP4 file path
        input: String,

        /// Output GIF file path
        #[clap(short, long)]
        output: Option<String>,

        /// Width to resize to (height will be adjusted automatically)
        #[clap(short, long)]
        width: Option<u32>,

        /// Frames per second for the output GIF
        #[clap(short, long, default_value = "10")]
        fps: u32,

        /// Maximum size of output GIF in MB
        #[clap(short, long, default_value = "5.0")]
        max_size: f64,

        /// Try multiple settings to achieve size target
        #[clap(long)]
        optimize: bool,
    },

    /// Split a 1920x1080 MP4 video into 5 equal vertical slices
    Splitter {
        /// Input MP4 file path
        input: String,

        /// Output directory for video slices
        #[clap(short, long, default_value = "output_slices")]
        output_dir: String,

        /// Prefix for output filenames
        #[clap(short, long, default_value = "slice")]
        prefix: String,

        /// Custom FFmpeg encoding options (advanced users only)
        #[clap(long)]
        custom_encode: Option<String>,

        /// Process even if video dimensions are not 1920x1080
        #[clap(long)]
        force: bool,
    },

    /// Merge video with audio
    Merger {
        /// Input video file path
        video: String,

        /// Input audio file path
        audio: String,

        /// Output file path
        #[clap(short, long)]
        output: String,

        /// End when shortest input stream ends
        #[clap(long)]
        shortest: bool,

        /// Copy codec without re-encoding (faster)
        #[clap(long)]
        copy_codec: bool,
    },
}

fn main() -> Result<(), eframe::Error> {
    // Check if FFmpeg is installed
    if !check_ffmpeg() {
        eprintln!("Error: FFmpeg is not installed or not found in PATH. Please install FFmpeg.");
        std::process::exit(1);
    }

    // Parse command-line arguments
    let cli = Cli::parse();

    // Run GUI if no subcommand is provided
    if cli.command.is_none() {
        let options = NativeOptions::default();
        return eframe::run_native(
            "Video-ToolKit",
            options,
            Box::new(|_cc| Box::new(VideoToolKitApp::default()))
        );
    }

    // Otherwise, run the appropriate command-line tool
    match cli.command.unwrap() {
        Commands::Clipper { input, ranges, output_dir, copy_codec, suffix } => {
            println!("Running clipper...");

            let time_ranges = parse_time_ranges(&ranges);
            if time_ranges.is_empty() {
                eprintln!("Error: No valid time ranges provided.");
                std::process::exit(1);
            }

            match clip_video(&input, &time_ranges, &output_dir, copy_codec, suffix.as_deref()) {
                Ok(true) => println!("Successfully extracted all {} clip(s).", time_ranges.len()),
                Ok(false) => {
                    eprintln!("Completed with some errors.");
                    std::process::exit(1);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },

        Commands::GifConverter { input, output, width, fps, max_size, optimize } => {
            println!("Running GIF converter...");

            // Determine output filename if not provided
            let output = match output {
                Some(o) => o,
                None => {
                    let input_path = std::path::Path::new(&input);
                    match input_path.file_stem() {
                        Some(stem) => {
                            let mut output_path = std::path::PathBuf::from(input_path.parent().unwrap_or_else(|| std::path::Path::new("")));
                            output_path.push(stem);
                            output_path.set_extension("gif");
                            output_path.to_string_lossy().to_string()
                        },
                        None => {
                            eprintln!("Error: Could not determine output filename.");
                            std::process::exit(1);
                        }
                    }
                }
            };

            let result = if optimize {
                optimize_conversion(&input, &output, max_size, width)
            } else {
                convert_mp4_to_gif(&input, &output, width, fps, max_size)
            };

            match result {
                Ok(true) => println!("Conversion successful! Output: {}", output),
                Ok(false) => {
                    eprintln!("Output file exceeds size limit (> {}MB).", max_size);
                    std::process::exit(1);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },

        Commands::Splitter { input, output_dir, prefix, custom_encode, force } => {
            println!("Running video splitter...");

            match split_video(&input, &output_dir, &prefix, custom_encode.as_deref(), force) {
                Ok(true) => println!("Successfully split video into 5 slices. Files saved in: {}", output_dir),
                Ok(false) => {
                    eprintln!("Completed with some errors.");
                    std::process::exit(1);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },

        Commands::Merger { video, audio, output, shortest, copy_codec } => {
            println!("Running audio/video merger...");

            match merge_audio_video(&video, &audio, &output, shortest, copy_codec) {
                Ok(_) => println!("Successfully merged audio and video. Output: {}", output),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },
    }

    Ok(())
}