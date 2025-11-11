# R-Edit

R-Edit is a lightweight, terminal-based text editor written in Rust. It is a simple yet functional editor built with a focus on learning and practicing Rust programming.

## Features

*   Basic text editing: Type, enter new lines, and use backspace.
*   Vim-like tilde `~` characters to indicate empty lines.
*   Clean, distraction-free user interface.
*   Graceful exit with a goodbye message.

## Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) programming language and Cargo package manager.

### Building and Running

1.  Clone the repository:
    ```sh
    git clone https://github.com/your-username/r-edit.git
    cd r-edit
    ```

2.  Build and run the project:
    ```sh
    cargo run
    ```

3.  To build a release version:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/r-edit`.

## Usage

Once the editor is running, you can start typing.

### Keybindings

*   `Ctrl-Q`: Quit the editor.

## License

This project is licensed under the terms of the LICENSE file.