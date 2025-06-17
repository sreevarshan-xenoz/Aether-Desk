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

- ⏰ **Wallpaper Scheduler**
  - Automatically change wallpapers based on time
  - Set up intervals for wallpaper rotation
  - Create custom triggers for wallpaper changes
  - Enable/disable individual schedule items

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

### Building Windows Executable

To build a Windows executable (.exe) file:

1. Make sure you have Rust and Cargo installed
2. Run the build script:
   ```bash
   .\build_windows.bat
   ```
   This will create a Windows executable in the `dist\windows` directory.

3. (Optional) To create a Windows installer:
   - Install [NSIS](https://nsis.sourceforge.io/Download) (Nullsoft Scriptable Install System)
   - Run the following command:
     ```bash
     makensis installer.nsi
     ```
   This will create an installer file named `Aether-Desk-Setup.exe` in the project root.

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

### Using the Wallpaper Scheduler

1. Click on the "Scheduler" tab
2. Click "Add Schedule Item" to create a new schedule
3. Configure the trigger type:
   - **Time**: Set a specific time of day (e.g., 8:00 AM)
   - **Interval**: Set a time interval (e.g., every 2 hours)
   - **System Event**: Trigger on system events (e.g., startup)
   - **Custom**: Create custom triggers

4. Select the wallpaper to display when the trigger activates
5. Enable or disable the schedule item
6. Click "Save" to add the schedule item

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
- [chrono](https://github.com/chronotope/chrono) for date and time handling

---

<div align="center">
Made with ❤️ by [SreeVarshan](https://github.com/sreevarshan-xenoz)
</div>

