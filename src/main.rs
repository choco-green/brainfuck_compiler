use std::fs::read_to_string;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    // Check if the user has provided a file, args[0] is the name of the program
    // and args[1] is the first argument provided by the user
    if args.len() != 2 {
        println!("Usage: brainfuck <filename> ");
        std::process::exit(1)
    }

    // Read from file and call the main entry point: run function
    let bytes = read_to_string(&Path::new(&args[1]))?;
    run(bytes);

    Ok(())
}

/// runs the given instruction with 1GB of memory
fn run(file: String) {
    let tokens = scan_tokens(file);
    let instructions = parse_tokens(tokens);

    let mut memory: Vec<u8> = vec![0; 1024];
    let mut pointer = 0;
    run_instructions(instructions, &mut memory, &mut pointer);
}

#[derive(Clone, Debug)]
enum Token {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Clone, Debug)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

/// Scanning the string and turning it into a vector of token
fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    for c in source.chars() {
        match c {
            '>' => tokens.push(Token::IncrementPointer),
            '<' => tokens.push(Token::DecrementPointer),
            '+' => tokens.push(Token::Increment),
            '-' => tokens.push(Token::Decrement),
            '.' => tokens.push(Token::Write),
            ',' => tokens.push(Token::Read),
            '[' => tokens.push(Token::LoopBegin),
            ']' => tokens.push(Token::LoopEnd),
            _ => {}
        };
    }

    tokens
}

/// Parsing the tokens into instructions
fn parse_tokens(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    // Counts the number of '[', thus the number of loops
    let mut loop_count = 0;

    // The index of the current token we are parsing
    let mut current = 0;

    // The index of the start of the loop
    let mut loop_start = 0;

    while current != tokens.len() {
        if loop_count == 0 {
            match tokens[current] {
                Token::IncrementPointer => instructions.push(Instruction::IncrementPointer),
                Token::DecrementPointer => instructions.push(Instruction::DecrementPointer),
                Token::Increment => instructions.push(Instruction::Increment),
                Token::Decrement => instructions.push(Instruction::Decrement),
                Token::Write => instructions.push(Instruction::Write),
                Token::Read => instructions.push(Instruction::Read),
                Token::LoopBegin => {
                    loop_start = current;
                    loop_count += 1;
                }
                // If we encounter a ']' without a matching '['
                Token::LoopEnd => panic!("Unexpected ']'"),
            }
        } else {
            // If we are inside a loop
            match tokens[current] {
                Token::LoopBegin => loop_count += 1,
                Token::LoopEnd => {
                    loop_count -= 1;
                    if loop_count == 0 {
                        instructions.push(Instruction::Loop(parse_tokens(
                            tokens[loop_start + 1..current].to_vec(),
                        )));
                    }
                }
                _ => {}
            }
        }
        current += 1;
    }

    // If we encountered a '[' without a matching ']'
    if loop_count != 0 {
        panic!("Expected ']'")
    }

    instructions
}

/// Running the instructions
fn run_instructions(instructions: Vec<Instruction>, memory: &mut Vec<u8>, pointer: &mut usize) {
    // The instruction pointer is set to 0 whilst recurring over new
    // instructions
    let mut instruction_pointer = 0;

    while instruction_pointer < instructions.len() {
        match &instructions[instruction_pointer] {
            Instruction::IncrementPointer => *pointer += 1,
            Instruction::DecrementPointer => *pointer -= 1,
            Instruction::Increment => memory[*pointer] += 1,
            Instruction::Decrement => memory[*pointer] -= 1,
            Instruction::Write => print!("{}", memory[*pointer] as char),
            Instruction::Read => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                memory[*pointer] = input.chars().next().unwrap() as u8;
            }
            Instruction::Loop(loop_instructions) => {
                while memory[*pointer] != 0 {
                    run_instructions(loop_instructions.to_vec(), memory, pointer);
                }
            }
        }
        instruction_pointer += 1;
    }
}
