# Video Toolkit

A comprehensive toolkit for video processing tasks, built in Rust.

## Features

- **Video Clipper**: Extract specific time segments from MP4 videos
- **GIF Converter**: Convert MP4 videos to optimized GIF format
- **Video Splitter**: Split a 1920x1080 video into 5 equal vertical slices
- **Audio/Video Merger**: Merge video with audio from different sources

## Requirements

- Rust (latest stable version)
- FFmpeg (must be installed and available in your PATH)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/video-toolkit.git
cd video-toolkit
```

2. Build the project:

```bash
cargo build --release
```

## Usage

### GUI Mode

To launch the graphical user interface:

```bash
cargo run --release
```

### Command-Line Interface

The toolkit can also be used from the command line:

#### Video Clipper

```bash
cargo run --release -- clipper --input video.mp4 --ranges "00:01:00-00:02:00" "00:05:30-00:06:15" --output-dir output_clips
```

Options:
- `--input`: Input MP4 file path
- `--ranges` or `-r`: Time ranges to extract in format START-END (e.g., 00:01:00-00:02:00)
- `--output-dir` or `-o`: Output directory for video clips (default: output_clips)
- `--copy-codec`: Copy codec instead of re-encoding (faster but may be less precise)
- `--suffix` or `-s`: Optional suffix to add to output filenames

#### GIF Converter

```bash
cargo run --release -- gif-converter --input video.mp4 --output output.gif --width 480 --fps 10
```

Options:
- `--input`: Input MP4 file path
- `--output` or `-o`: Output GIF file path
- `--width` or `-w`: Width to resize to (height adjusted automatically)
- `--fps` or `-f`: Frames per second (default: 10)
- `--max-size` or `-s`: Maximum output size in MB (default: 5.0)
- `--optimize`: Try multiple settings to achieve target size

#### Video Splitter

```bash
cargo run --release -- splitter --input video.mp4 --output-dir output_slices --prefix slice
```

Options:
- `--input`: Input MP4 file path
- `--output-dir` or `-o`: Output directory (default: output_slices)
- `--prefix` or `-p`: Prefix for output filenames (default: slice)
- `--custom-encode`: Custom FFmpeg encoding options
- `--force`: Process even if video dimensions are not 1920x1080

#### Audio/Video Merger

```bash
cargo run --release -- merger --video video.mp4 --audio audio.mp3 --output merged.mp4 --shortest --copy-codec
```

Options:
- `--video`: Input video file path
- `--audio`: Input audio file path
- `--output` or `-o`: Output file path
- `--shortest`: End when shortest input stream ends
- `--copy-codec`: Copy codec without re-encoding (faster)

## Project Structure

This project uses a workspace structure:

- `common`: Shared utilities and error handling
- `clipper`: Video clipping functionality
- `gif_converter`: MP4 to GIF conversion
- `splitter`: Video splitting functionality
- `merger`: Audio/video merging functionality
- `ui`: GUI components using egui

## License

MIT

## Acknowledgements

This project uses [FFmpeg](https://ffmpeg.org/) for all video processing tasks.