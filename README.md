# desunu

**desunu** (*unused* backward) is a blazing-fast, Rust-based CLI tool designed to help you locate unused dependencies in your projects. Whether you're working with `package.json` for Node.js, `Cargo.toml` for Rust, or eventually other project types, **desunu** scans your files, analyzes your dependencies, and provides actionable insights. 

Our vision is simple: **desunu** should be the *first tool you think of* when you need to find unused things.

## Features

- **Comprehensive Dependency Scanning**: Detect every unused dependency in your project, starting with Node.js (`package.json`) and Rust (`Cargo.toml`).
- **Blazing Performance**: Analyze projects with 250+ packages and 2300+ files in less than 5 seconds.
- **File-Based Analysis**: Reads and scans your files directly without relying on external services like LSP.
- **Future-Ready**: Designed for extensibility, with plans to support additional ecosystems like Python, Go, and Ruby.

## Why Use desunu?

- Keep your projects clean and lean by identifying unnecessary dependencies.
- Avoid performance issues and vulnerabilities caused by unused libraries.
- Focus on simplicity and speed: no complex setups, just results.

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your system.

### Steps
1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/desunu.git
    ```

2. Navigate to the project directory:
    ```bash
    cd desunu
    ```

3. Build the project:
    ```bash
    cargo build --release
    ```

4. Run the executable from the `target/release/` folder to start using desunu.

## Usage

### Basic Usage

To scan your project directory and detect unused dependencies, run:

```bash
desunu all
```

desunu will:

1. Detect in which project type you are (Node, Rust, Ruby, etc...).
2. Analyze the files in your project directory (and subdirectories).
3. Cross-reference your dependency file (package.json, Cargo.toml) with the actual imports or usage in your code.
4. Output a clear list of unused dependencies.

## Contributing

We welcome contributions! If you'd like to improve spedesunu or add support for additional project types, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Make your changes and commit them (`git commit -am 'Add feature'`).
4. Push to your forked repository (`git push origin feature-name`).
5. Open a pull request with a clear description of your changes.

## Roadmap

- Publish the CLI on `brew`, `apt`.
- Add support for Python, Go, Ruby, and more project types.
- Customizable configurations for ignoring specific files or dependencies.
- Advanced reporting formats (JSON, CSV, etc.).
- Integration into CI/CD pipelines for automated checks.

## Performance

**desunu** is optimized for speed. It can scan large projects with thousands of files and hundreds of dependencies in just a few seconds, making it an ideal tool for both developers and CI environments.

## License

**desunu** is licensed under the MIT License.

## Acknowledgments

Built with ❤️ and Rust.

