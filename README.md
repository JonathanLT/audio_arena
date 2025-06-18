# Audio Arena

Audio Arena is a cross-platform audio player built in Rust. This project allows users to load, shuffle, and play audio files from a specified directory. It utilizes the `walkdir` library for file handling and the `rodio` library for audio playback.

## Features

- Load audio files from a specified directory.
- List available audio files.
- Shuffle the audio files for random playback.
- Play audio files with a countdown timer.

## Dependencies

This project relies on the following libraries:

- `walkdir`: For traversing directories and loading audio files.
- `rodio`: For audio playback.
- `druid` (or another chosen graphical library): For creating a cross-platform graphical user interface.

## Installation

To run this project, ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and navigate to the project directory:

```bash
git clone <repository-url>
cd audio_arena
```

Then, build and run the project:

```bash
cargo run
```

## Usage

1. Place your audio files in the `musics` directory.
2. Run the application, and it will load the audio files, shuffle them, and play each file in sequence.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.