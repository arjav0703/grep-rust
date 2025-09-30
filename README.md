## Grep

This project is a simple implementation of the `grep` command in Rust. It allows you to search for a specific pattern in a file and print the lines that contain the pattern.


### Usage
1. Make sure you have Rust installed on your machine. If you don't have it yet, you can install it from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository:

```bash
git clone https://github.com/arjav0703/grep-rust.git
```

3. Navigate to the project directory:

```bash
cd grep-rust
```

4. To use the program, run the following command:

```bash
cargo run -- -E "pattern" filename
```

### Features:
- Prints lines that match the given pattern.
- Support advanced regex patterns like `\d`, `\w`, etc.
