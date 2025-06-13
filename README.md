# Aether Renderer Core 🌌

![Aether Renderer Core](https://img.shields.io/badge/Download-Release-brightgreen?style=for-the-badge&logo=github)

Welcome to **Aether Renderer Core**, a powerful Rust command-line interface (CLI) designed to convert PNG and WebP image sequences into transparent .webm or .mp4 videos. This tool is perfect for creative layering and visual rituals, with a future GUI in the works. 

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Topics](#topics)
- [Contributing](#contributing)
- [License](#license)
- [Support](#support)

## Features

- **Format Support**: Convert PNG and WebP sequences into .webm or .mp4 formats.
- **Alpha Transparency**: Maintain transparency for overlay animations.
- **Creative Tools**: Ideal for artists and developers looking to enhance their visual projects.
- **Future GUI**: Plans for a graphical user interface to simplify usage.
- **Rust Performance**: Built with Rust for speed and reliability.

## Installation

To get started with Aether Renderer Core, you need to download the latest release. Visit the [Releases](https://github.com/hyhloc/aether-renderer-core/releases) section to find the appropriate version for your system. Download the file, then execute it to install the tool.

## Usage

Using Aether Renderer Core is straightforward. After installation, you can run the CLI with the following command:

```bash
aether-renderer --input <input_sequence> --output <output_file> --format <webm|mp4> --alpha
```

### Parameters

- `--input`: Specify the path to your image sequence (PNG or WebP).
- `--output`: Define the name of the output video file.
- `--format`: Choose between `webm` or `mp4` formats.
- `--alpha`: Include this flag to maintain alpha transparency.

### Example Command

Here’s an example of how to convert a sequence of PNG images into a transparent .webm video:

```bash
aether-renderer --input images/frame_%04d.png --output output/video.webm --format webm --alpha
```

## Examples

### Creating a Transparent Video

1. Prepare your image sequence named `frame_0001.png`, `frame_0002.png`, etc.
2. Run the following command:

```bash
aether-renderer --input images/frame_%04d.png --output output/video.webm --format webm --alpha
```

3. Your output video will be created in the specified output directory.

### Converting WebP Sequences

You can also convert WebP image sequences in a similar manner:

```bash
aether-renderer --input images/frame_%04d.webp --output output/video.mp4 --format mp4 --alpha
```

## Topics

Aether Renderer Core is related to several topics in the creative and tech space:

- **alpha-channel**
- **cli**
- **creative-tools**
- **ffmpeg**
- **frame-sequence**
- **png**
- **renderer**
- **rust**
- **rust-lang**
- **sacred**
- **sacred-ai**
- **sacred-ai-art**
- **transparent-video**
- **video-tools**
- **webm**

## Contributing

We welcome contributions to Aether Renderer Core. If you have ideas for features or improvements, please open an issue or submit a pull request. Ensure that your code follows our coding standards and includes tests.

### Steps to Contribute

1. Fork the repository.
2. Create a new branch for your feature.
3. Make your changes.
4. Write tests if applicable.
5. Submit a pull request with a description of your changes.

## License

Aether Renderer Core is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

For support, please check the [Releases](https://github.com/hyhloc/aether-renderer-core/releases) section for updates. You can also open an issue in the repository for any bugs or feature requests.

---

Thank you for using Aether Renderer Core. We hope it enhances your creative projects and workflows. If you have any questions, feel free to reach out or contribute to the project. Happy rendering!