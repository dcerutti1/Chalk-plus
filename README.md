# Chalk-rs
a simple port of the library "chalk.js" to Rust

Chalk.rs
Chalk.rs is a Rust library that provides a simple and flexible way to style text in the terminal. Whether you're building command-line applications, making scripts more colorful, or simply want to enhance your terminal output, Chalk.rs lets you easily apply text styles and colors.

Features
Colors: Apply colors to your text, including Black, Red, Green, Yellow, Blue, Magenta, Cyan, and White.

Text Styles: Bold, Italic, Underline, Strikethrough, Overline, and Inverse for various text effects.

Easy-to-Use API: Chain methods for styling text (e.g., chalk.red("Hello").bold().underline().display()).

Cross-Platform: Works across different platforms that support ANSI escape codes (Linux, macOS, and Windows with appropriate terminal support).

Installation
To add Chalk.rs to your project, add it to your Cargo.toml dependencies:

toml
Copy
Edit
[dependencies]
chalk = "0.1"
Basic Usage
Here’s a quick example of how to use Chalk.rs in your Rust application:

```rust
use chalk::Chalk;

fn main() {
    let chalk = Chalk::new();
    chalk.red("Hello, world!").bold().underline().display();
}
```

Available Styles and Colors
Colors:

chalk.red()

chalk.green()

chalk.blue()

chalk.yellow()

chalk.magenta()

chalk.cyan()

chalk.white()

Text Styles:

.bold()

.dim()

.italic()

.underline()

.inverse()

.strikethrough()

.overline()


Reset After Each Use
Each style and color is automatically reset after use, ensuring that subsequent terminal output is unaffected by previous styling.

Contributing
Contributions are welcome! If you have suggestions, bug fixes, or new features, feel free to open an issue or submit a pull request. Here's how you can contribute:

Fork the repository.

Create a new branch (git checkout -b feature-branch).

Make your changes.

Commit your changes (git commit -am 'Add new feature').

Push to the branch (git push origin feature-branch).

Open a pull request.

License
This project is licensed under the MIT License - see the LICENSE file for details.

Acknowledgements
Inspired by libraries like chalk.js (JavaScript) for text styling in the terminal.
