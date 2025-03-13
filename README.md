## About rtree

rtree is a robust command-line tool written in Rust designed to help visualize and manage directory structures. This tool provides a fast and intuitive way to display project hierarchies, making it easy to navigate large codebases.

- **Fast Execution:** Built in Rust for high performance.
- **User-Friendly:** Clear and concise output for easy understanding.
- **Customizable:** Adapt the view to suit your project organization.
- **Color-Coded Timestamps:** Clearly distinguishes time accessed and created with color-coded indicators for quick visual reference.

- **Fast Execution:** Built in Rust for high performance.
- **User-Friendly:** Clear and concise output for easy understanding.
- **Customizable:** Adapt the view to suit your project organization.

## Installation

1. **Clone the Repository:**

    ```bash
    git clone https://github.com/michaelcolletti/rtree.git
    cd rtree
    ```

2. **Build the Project:**

    Make sure you have Rust installed, then run:
    
    ```bash
    cargo build --release
    ```

## Usage

Run the tool directly from the command-line:

```bash
cargo run --release
```

Or run the compiled binary: 

```bash
./target/release/rtree [OPTIONS]
```

### Command-Line Options

- `-h, --help`  
  Display the help information about rtree usage.

- `-V, --version`  
  Show the version of the tool.

- Additional options might be available—check the help message for details.

## Code Documentation

The main application logic is located in `src/main.rs`. Key components include:

- **Parsing Input**  
  The entry point handles command-line arguments and ensures the correct parameters are supplied.

- **Tree Traversal**  
  rtree uses efficient algorithms to scan directories and format the output as a tree.

- **Error Handling**  
  The program provides clear error messages to guide users in case of invalid inputs or permissions issues.

For a detailed walkthrough, refer to the inline comments in `src/main.rs`.

## Contributing

Contributions are welcome! If you'd like to report a bug or request a feature, please open an issue. Pull requests are appreciated.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/fooBar`).
3. Commit your changes.
4. Push to the branch (`git push origin feature/fooBar`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

Inspired by the classic Linux tree command—an essential tool for visualizing directory hierarchies—rtree adopts its core concept of displaying file structures in a clear, indented tree format. The Linux tree command has long been a trusted utility for system administrators and developers to quickly understand directory layouts. Building on this proven concept, rtree enhances performance, offers custom output tailoring, and integrates seamlessly into modern development workflows.

Enjoy using rtree and happy coding!
