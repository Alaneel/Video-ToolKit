# Video-ToolKit

A comprehensive toolkit for video processing tasks, built in Rust.

## Features

- **Video Clipper**: Extract specific time segments from video files
- **GIF Converter**: Convert videos to optimized GIF format
- **GIF Transparency**: Batch process GIFs to make backgrounds transparent
- **Video Splitter**: Split a video into equal vertical slices
- **Audio/Video Merger**: Merge video with audio from different sources
- **Batch Processing**: Process multiple files in one operation
- **Profile System**: Save and load operation settings
- **Plugin Architecture**: Extend functionality with third-party plugins
- **Multi-Format Support**: Works with a wide range of video, audio, and image formats

## Requirements

- Rust (latest stable version)
- FFmpeg (must be installed and available in your PATH)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/Video-ToolKit.git
cd Video-ToolKit
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

The GUI provides access to all features including video operations, batch processing, profiles, and plugin management.

### Command-Line Interface

The toolkit can also be used from the command line:

#### Video Clipper

```bash
cargo run --release -- clipper --input video.mp4 --ranges "00:01:00-00:02:00" "00:05:30-00:06:15" --output-dir output_clips
```

Options:
- `--input`: Input video file path
- `--ranges` or `-r`: Time ranges to extract in format START-END (e.g., 00:01:00-00:02:00)
- `--output-dir` or `-o`: Output directory for video clips (default: output_clips)
- `--copy-codec`: Copy codec instead of re-encoding (faster but may be less precise)
- `--suffix` or `-s`: Optional suffix to add to output filenames
- `--format`: Output format (e.g., mp4, mkv, avi)

#### GIF Converter

```bash
cargo run --release -- gif-converter --input video.mp4 --output output.gif --width 480 --fps 10
```

Options:
- `--input`: Input video file path
- `--output` or `-o`: Output GIF file path
- `--width` or `-w`: Width to resize to (height adjusted automatically)
- `--fps` or `-f`: Frames per second (default: 10)
- `--max-size` or `-s`: Maximum output size in MB (default: 5.0)
- `--optimize`: Try multiple settings to achieve target size

#### GIF Transparency

```bash
cargo run --release -- gif-transparency input1.gif input2.gif --backup
```

Or process a directory:

```bash
cargo run --release -- gif-transparency-dir ./gifs --recursive --backup
```

Options:
- `--recursive` or `-r`: Process subdirectories recursively
- `--backup` or `-b`: Create backup of original files before processing

#### Video Splitter

```bash
cargo run --release -- splitter --input video.mp4 --output-dir output_slices --prefix slice
```

Options:
- `--input`: Input video file path
- `--output-dir` or `-o`: Output directory (default: output_slices)
- `--prefix` or `-p`: Prefix for output filenames (default: slice)
- `--custom-encode`: Custom FFmpeg encoding options
- `--force`: Process even if video dimensions are not 1920x1080
- `--format`: Output format (e.g., mp4, mkv, avi)

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
- `--format`: Output format (e.g., mp4, mkv, avi)

#### Batch Processing

Process multiple files with a single command:

```bash
cargo run --release -- batch clipper ./videos --recursive --ranges "00:01:00-00:02:00" --output-dir output_clips
```

General batch options:
- `--recursive` or `-r`: Process directories recursively
- `--pattern` or `-p`: File pattern to match (regex)
- `--parallel`: Process files in parallel (default: true)

See CLI help for operation-specific options.

#### Profile Management

Save, load, and manage operation profiles:

```bash
cargo run --release -- profile create --name "my_profile" --profile-type clipper --params "output_dir=output_clips" "copy_codec=true"
```

Profile commands:
- `list`: List available profiles
- `show`: Show a specific profile
- `create`: Create a new profile
- `delete`: Delete a profile
- `import`: Import a profile from a file
- `export`: Export a profile to a file

#### Plugin Management

Work with plugins to extend functionality:

```bash
cargo run --release -- plugin discover
```

Plugin commands:
- `list`: List available plugins
- `load`: Load a plugin from a file
- `run`: Run a plugin with parameters
- `discover`: Discover and load plugins from the default plugin directory

#### Format Support

List supported formats:

```bash
cargo run --release -- formats
```

## Project Structure

This project uses a workspace structure:

- `common`: Shared utilities and error handling
- `clipper`: Video clipping functionality
- `gif_converter`: Video to GIF conversion
- `gif_transparency`: GIF transparency processing
- `splitter`: Video splitting functionality
- `merger`: Audio/video merging functionality
- `batch_processing`: Multi-file processing capabilities
- `profile_system`: Save and load operation settings
- `plugin_system`: Plugin architecture for extensions
- `ui`: GUI components using egui

## Custom Plugins

You can create your own plugins to extend Video-ToolKit. A plugin is a dynamic library that implements the `Plugin` trait:

1. Create a new library project:
```bash
cargo new --lib my_plugin
```

2. Configure it as a dynamic library in `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
plugin_system = { path = "/path/to/Video-ToolKit/crates/plugin_system" }
```

3. Implement the Plugin trait:
```rust
use plugin_system::{Plugin, PluginMetadata, PLUGIN_API_VERSION};

struct MyPlugin { /* ... */ }

impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "my_plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "Your Name".to_string(),
            description: "My custom plugin".to_string(),
            api_version: PLUGIN_API_VERSION,
        }
    }
    
    // Implement other required methods...
}

// Export the plugin
plugin_system::export_plugin!(MyPlugin);
```

4. Build the plugin:
```bash
cargo build --release
```

5. Copy the compiled library to the Video-ToolKit plugins directory.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT

## Acknowledgements

This project uses [FFmpeg](https://ffmpeg.org/) for all video processing tasks.