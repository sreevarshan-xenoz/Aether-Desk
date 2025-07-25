# Contributing to Aether-Desk

Thank you for your interest in contributing to Aether-Desk! This document provides guidelines for contributing to the project.

## 🚀 Getting Started

### Prerequisites

- **Rust** (latest stable version)
- **Git**
- Platform-specific dependencies:
  - **Linux**: `libgtk-3-dev`, `libxrandr-dev`, `libxss1`, `libasound2-dev`
  - **Windows**: Windows 10/11 with Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools

### Setting up the Development Environment

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sreevarshan-xenoz/aether-desk.git
   cd aether-desk
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```

## 📝 Development Guidelines

### Code Style

- Follow Rust's official style guidelines
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write meaningful commit messages

### Testing

- Write unit tests for new functionality
- Include integration tests for complex features
- Ensure all tests pass before submitting PR
- Aim for good test coverage

### Performance

- Consider performance implications of changes
- Use the built-in performance monitoring tools
- Test on multiple platforms when possible

## 🔄 Development Workflow

### Branch Naming

- `feature/description` - New features
- `bugfix/description` - Bug fixes
- `refactor/description` - Code refactoring
- `docs/description` - Documentation updates

### Pull Request Process

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes and commit:**
   ```bash
   git add .
   git commit -m "Add: your feature description"
   ```

3. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create a Pull Request:**
   - Provide a clear description of changes
   - Reference any related issues
   - Include screenshots for UI changes
   - Ensure CI passes

### Commit Messages

Follow the conventional commit format:

- `Add: new feature or functionality`
- `Fix: bug fixes`
- `Update: improvements to existing features`
- `Remove: deletion of features or code`
- `Docs: documentation changes`
- `Style: formatting changes`
- `Refactor: code refactoring`
- `Test: adding or updating tests`

## 🏗️ Project Structure

```
aether-desk/
├── src/
│   ├── core/           # Core application logic
│   ├── platform/       # Platform-specific implementations
│   ├── ui/            # User interface components
│   ├── wallpapers/    # Wallpaper management
│   └── main.rs        # Application entry point
├── tests/             # Integration tests
├── assets/            # Application assets
├── config/            # Configuration files
└── docs/              # Documentation
```

## 🐛 Reporting Issues

When reporting issues, please include:

- **OS and version**
- **Rust version** (`rustc --version`)
- **Steps to reproduce**
- **Expected behavior**
- **Actual behavior**
- **Screenshots** (if applicable)
- **Log output** (if relevant)

## 💡 Feature Requests

For feature requests:

- Check existing issues first
- Provide clear use cases
- Explain why the feature would be beneficial
- Consider implementation complexity

## 🧪 Testing Guidelines

### Unit Tests

- Place tests in the same file as the code (`#[cfg(test)]` module)
- Test both success and failure cases
- Use descriptive test names

### Integration Tests

- Place in the `tests/` directory
- Test complete workflows
- Mock external dependencies when necessary

### Performance Tests

- Use the performance monitoring tools
- Test memory usage and CPU performance
- Consider different wallpaper types and sizes

## 📚 Documentation

- Update documentation for new features
- Include code examples where helpful  
- Keep README.md up to date
- Comment complex algorithms

## 🔒 Security

If you discover a security vulnerability:

1. **Do not** open a public issue
2. Email the maintainers directly
3. Include detailed steps to reproduce
4. Allow time for a fix before public disclosure

## 📄 License

By contributing to Aether-Desk, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be recognized in:

- The project README
- Release notes for significant contributions
- The project's website (when available)

## ❓ Questions

If you have questions:

- Check the existing documentation
- Search through existing issues
- Ask in GitHub Discussions
- Contact the maintainers

Thank you for contributing to Aether-Desk! 🌟
