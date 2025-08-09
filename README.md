# ccwc - Coding Challenge Word Count

`ccwc` is a simple command-line utility written in Rust, inspired by the Unix `wc` tool. It serves as a solution to the "Build Your Own `wc` Tool" challenge from [codingchallenges.fyi](https://codingchallenges.fyi/challenges/challenge-wc).

This tool can count bytes, lines, words, and characters from a specified file or from standard input.

## Features

-   Count bytes in a file (`-c` or `--bytes`).
-   Count lines in a file (`-l` or `--lines`).
-   Count words in a file (`-w` or `--words`).
-   Count characters in a file (`-m` or `--characters`).
-   Provide a default summary (lines, words, bytes, and characters) when no options are specified.
-   Read content from standard input (`stdin`) when no file is provided.

## Installation

Before you begin, ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system.

1.  **Clone the repository or download the source code.**
2.  **Navigate to the project's root directory:**
    ```bash
    cd /path/to/your/project
    ```
3.  **Build and install the binary:**
    This command compiles the source code and places the resulting executable (`ccwc`) into your Cargo bin path (`~/.cargo/bin`), making it available as a command throughout your system.
    ```bash
    cargo install --path .
    ```
4.  **Verify the installation:**
    You should now be able to run the `ccwc` command.
    ```bash
    ccwc --version
    ```

## Usage

You can use `ccwc` by providing a command-line option and a file path. If no file path is given, it will read from standard input.

First, download the test file for the examples below:
```bash
curl [https://codingchallenges.fyi/challenges/challenge-wc/test.txt](https://codingchallenges.fyi/challenges/challenge-wc/test.txt) -o test.txt
```

### Count Bytes (`-c`)
Outputs the total number of bytes in the file.
```bash
> ccwc -c test.txt
342190 test.txt
```

### Count Lines (`-l`)
Outputs the total number of lines (specifically, newlines) in the file.
```bash
> ccwc -l test.txt
7145 test.txt
```

### Count Words (`-w`)
Outputs the total number of words, delimited by whitespace.
```bash
> ccwc -w test.txt
58164 test.txt
```

### Count Characters (`-m`)
Outputs the total number of characters. This may differ from the byte count for files containing multi-byte characters.
```bash
> ccwc -m test.txt
339292 test.txt
```

### Default (No Options)
When no options are provided, `ccwc` outputs the byte, line, word, and character counts, followed by the filename.
```bash
> ccwc test.txt
342190 7145 58164 339292 test.txt
```
*(Note: The order is byte, line, word, character)*

### Reading from Standard Input (`stdin`)
The tool can be integrated into a pipeline. If no file is specified, it reads from `stdin`.
```bash
> cat test.txt | ccwc -l
7145
```bash
> cat test.txt | ccwc
342190 7145 58164 339292
