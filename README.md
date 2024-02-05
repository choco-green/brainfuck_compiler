# Brainfuck Interpreter in Rust

## Overview

This Rust program is a simple interpreter for the Brainfuck programming
language. As Brainfuck is known for its minimalistic design and consists of only
eight simple commands and an instruction pointer, I have set out to write a
simple compiler / interpreter for the language.

## Features

- Interpret Brainfuck code from a file.
- Provide 1GB of memory for program execution.
- Handle basic Brainfuck instructions including loops.

## Requirements

- Install Rust

## Installation and Running the Interpreter

1. Clone the repository or download the source code.
2. Navigate to the directory containing the code.
3. To run a Brainfuck program, use:
   ```
   cargo run <path_to_brainfuck_file>
   ```
   Replace `<path_to_brainfuck_file>` with the path to your Brainfuck code file.

## Usage

The program expects a single command line argument: the path to a Brainfuck source file. It will interpret and execute
the code in that file.

Example:

```
./brainfuck hello_world.bf
```

## Components

- `main`: Entry point of the program. Handles command line arguments and file reading.
- `run`: Main interpreter function. Converts Brainfuck code into tokens and instructions, and executes them.
- `scan_tokens`: Converts the Brainfuck code string into a vector of tokens.
- `parse_tokens`: Parses tokens into executable instructions.
- `run_instructions`: Executes the parsed Brainfuck instructions.
- `Token`: Enum representing the different types of Brainfuck tokens.
- `Instruction`: Enum representing executable Brainfuck instructions.

## Error Handling

- If no file is provided, or an incorrect number of arguments is given, the program will display a usage message and
  exit.
- The program will panic if it encounters unmatched loop symbols ('[' or ']').

## Contributing

Contributions to this interpreter are welcome. Please ensure to follow Rust's coding standards and provide comments
where necessary.

## License

This project is open source and available under the MIT License

## Disclaimer

This interpreter is a basic implementation and might not cover all edge cases found in more complex Brainfuck programs.
