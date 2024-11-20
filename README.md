# spedesunu

**spedesunu** is a lightweight Rust-based CLI tool designed to help you identify unused dependencies in your project. Currently focused on Node.js projects, **spedesunu** scans your files to find and report unused dependencies, but in the future, it will support multiple project types (e.g., Python, Go, Ruby, etc.).

---

## Features

- Detect unused dependencies in Node.js projects.
- Supports common JavaScript/TypeScript file extensions (`.js`, `.jsx`, `.ts`, `.tsx`) and more.
- Automatically excludes irrelevant files and directories (e.g., `node_modules/`).
- Simple and intuitive output highlighting unused dependencies.
- Planned extensibility for additional project types (Python, Go, Ruby, etc.).

---

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your system.

### Steps
1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/spedesunu.git
    ```

2. Navigate to the project directory:
    ```bash
    cd spedesunu
    ```

3. Build the project:
    ```bash
        cargo build --release
    ```

4. You can now run spedesunu with your Node.js project to detect unused dependencies.

## Usage
### Scan Your Node.js Project

To scan your project directory and detect unused dependencies, simply run:

```bash
spedesunu
```

This will scan the files in the current directory (and subdirectories), compare them with the dependencies in your `package.json`, and output which dependencies are unused.


## Contributing

We welcome contributions! If you'd like to improve spedesunu or add support for additional project types, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Make your changes and commit them (`git commit -am 'Add feature'`).
4. Push to your forked repository (`git push origin feature-name`).
5. Open a pull request with a clear description of your changes.

## Roadmap

- Support for multiple project types (Python, Go, Ruby, etc.)
- Improved file scanning for more efficient dependency detection
- Customizable configuration for ignoring specific files, extensions, etc.
- Enhanced reporting to display dependencies in different formats (JSON, CSV, etc.)

## License

**spedesunu** is licensed under the MIT License.

## Acknowledgments

Built with ❤️ and Rust.

