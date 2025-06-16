# Aether-Desk 🌟

<div align="center">

![Aether-Desk Logo](https://via.placeholder.com/150?text=Aether-Desk)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-blue)](https://github.com/sreevarshan-xenoz/aether-desk)

*A modern wallpaper engine for Windows and Linux, written in Rust*

</div>

## ✨ Features

- 🖼️ **Multiple Wallpaper Types**
  - Static images (PNG, JPG, BMP, GIF)
  - Video wallpapers (MP4, WebM, AVI, MKV)
  - Web-based wallpapers (HTML5)
  - Shader-based wallpapers (GLSL)
  - Audio-reactive visualizations

- 🔄 **Cross-Platform Support**
  - Windows 10/11
  - Linux (GNOME, KDE, XFCE, etc.)

- 🎨 **Modern UI**
  - Clean, intuitive interface
  - Easy wallpaper selection and management
  - Real-time preview

- ⚡ **Performance**
  - Low resource usage
  - Hardware acceleration when available
  - Efficient memory management

- 🔌 **Extensibility**
  - Plugin system (coming soon)
  - Custom wallpaper types
  - API for external control

## 📥 Installation

### Windows

1. Download the latest release from the [Releases](https://github.com/sreevarshan-xenoz/aether-desk/releases) page
2. Extract the ZIP file to a location of your choice
3. Run `aether-desk.exe`

### Linux

1. Download the latest release from the [Releases](https://github.com/sreevarshan-xenoz/aether-desk/releases) page
2. Extract the TAR.GZ file to a location of your choice
3. Run `./aether-desk`

## 🛠️ Building from Source

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Git

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/sreevarshan-xenoz/aether-desk.git
   cd aether-desk
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## 🚀 Usage

1. Launch the application
2. Select the type of wallpaper you want to use:
   - **Static**: Images (PNG, JPG, BMP, GIF)
   - **Video**: Video files (MP4, WebM, AVI, MKV)
   - **Web**: Web pages (URL)
   - **Shader**: GLSL shaders
   - **Audio**: Audio-reactive shaders

3. Choose a file or enter a URL
4. Click "Apply" to set the wallpaper
5. Click "Stop" to clear the wallpaper

## 📋 Dependencies

| Wallpaper Type | Dependencies |
|----------------|--------------|
| Static | None required |
| Video | VLC media player |
| Web | Edge (Windows) / Firefox (Linux) |
| Shader | Shader player |
| Audio | Shader player with audio visualization |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [egui](https://github.com/emilk/egui) for the UI framework
- [rfd](https://github.com/PolyMeilex/rfd) for the file dialog
- [serde](https://github.com/serde-rs/serde) for serialization
- [log](https://github.com/rust-lang/log) for logging

---

<div align="center">
Made with ❤️ by [Sree Varshan V](https://github.com/sreevarshan-xenoz)
</div>

