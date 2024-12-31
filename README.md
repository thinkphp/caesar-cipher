# Caesar Cipher Application

This Rust application reads a text from an input file, applies the Caesar cipher encoding algorithm, and writes the encoded text to an output file. The shift value for the cipher can be customized via command-line arguments.

## Features

- Reads plaintext from a specified input file.
- Writes encoded text to a specified output file.
- Allows setting a custom shift value for the Caesar cipher.
- Handles command-line arguments for flexibility.

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.

## Installation

Clone this repository and navigate to the project directory:

```bash
git clone https://github.com/your-username/caesar-cipher.git
cd caesar-cipher
```

Build the project using `cargo`:

```bash
cargo build --release
```

## Usage

Run the application from the command line:

```bash
cargo run -- <shift> <input_file> <output_file>
```

- `<shift>`: The number of positions to shift characters in the Caesar cipher.
- `<input_file>`: Path to the file containing plaintext.
- `<output_file>`: Path where the encoded text will be written.

### Example

Suppose you have a file named `input.txt` containing:

```
Hello, World!
```

Run the application as follows:

```bash
cargo run -- 3 input.txt output.txt
```

The file `output.txt` will contain:

```
Khoor, Zruog!
```

## Error Handling

- If the input file does not exist or cannot be read, the program will print an error message.
- If the output file cannot be written, an appropriate error message will be displayed.

## How It Works

1. The program reads the text from the specified input file.
2. Each character is shifted by the specified number of positions in the ASCII table:
   - Alphabetic characters are shifted cyclically.
   - Non-alphabetic characters remain unchanged.
3. The encoded text is written to the output file.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Happy coding! 🎉
``` 
