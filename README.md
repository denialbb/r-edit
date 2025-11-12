# R-Edit

R-Edit is a lightweight, terminal-based text editor written in Rust. It is a simple yet functional editor built with a focus on learning and practicing Rust programming.

## References

- Thank you to @pflenker for the clear and nice to follow guide on
  building an editor, check it out[^1]

## Features

- Basic text editing: Type, enter new lines, and use backspace.
- Cursor movement: Arrow keys, Home, End, PageUp, and PageDown.
- Vim-like tilde `~` characters to indicate empty lines.
- Clean, distraction-free user interface.
- Graceful exit with a goodbye message.
- Logging to a file (`r-edit.log`).
- File I/O: Open and save files.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) programming language and Cargo package manager.

### Building and Running

1.  Clone the repository and navigate to the project directory.

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

- `Ctrl-Q`: Quit the editor.
- `Ctrl-S`: Save the current file.
- `Char`: Any character is printed to the screen.
- `Enter`: Creates a new line.
- `Backspace`: Deletes the character to the left of the cursor.
- `Arrow Keys`: Move the cursor up, down, left, or right.
- `Home`: Moves the cursor to the beginning of the line.
- `End`: Moves the cursor to the end of the line.
- `PageUp`: Moves the cursor to the top of the screen.
- `PageDown`: Moves the cursor to the bottom of the screen.

## Development

To see the editor's logs, you can tail the `r-edit.log` file:

```sh
tail -f r-edit.log
```

## License

This project is licensed under the terms of the LICENSE file.

[^1]: (https://philippflenker.com/hecto/)
