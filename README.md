# Rust-Traverse

Rust traverse is a terminal based file explorer. It is inspired by the [NNN](https://github.com/jarun/nnn) file manager. It uses [Ratatui](https://github.com/tui-rs-revival/ratatui) for the terminal UI, with Crossterm for the terminal backend.

> To traverse or not to traverse?

## Features

- [x] Full CRUD operations on files and directories.
- [x] Keyboard shortcuts for navigation and operations, to make sure you don't have to leave the keyboard.
- [x] Traverse directly to a directory by typing its path.
- [x] Configurable.
- [x] Fuzzy finder for files in your current directory.
- [x] Preview files in the terminal.
- [x] Blazingly fast.
- [x] Cross platform.

## Installation

### From source

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository.
3. Run `cargo build --release`.
4. The binary will be in `target/release/trav`.
5. Add the binary to your path.

### From binaries

1. TODO (I'll add them soon).

## Usage

Run `trav` in your terminal.

### Keyboard Shortcuts

#### Navigation

- `ESC`: Quit the application.
- `1`: Select the Files pane.
- `2`: Select the Directories pane.
- `j`: Select the next item in the current pane.
- `k`: Select the previous item in the current pane.

#### File and Directory Operations

- `n`: Create a new file or directory, depending on the current pane.
- `CTRL + d`: Delete the selected file or directory, (to bin).
- `r`: Rename the selected file or directory.
- `f`: Navigate to a directory using a relative or absolute path.

#### Fuzzy Finder Operations

- `w`: Toggle fzf.
- `CTRL + n`: 'Next' item in results.
- `CTRL + p`: 'Previous' item in results.

## Configuration

TODO
