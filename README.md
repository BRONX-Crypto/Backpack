# Backpack VM

A simple stack-based virtual machine written in Rust.

Backpack reads a binary program (as a string of `0`s and `1`s) from standard input, tokenizes it, and executes it on a bit-level stack.

## Features

- Stack-based architecture
- Bit-level operations
- Support for arithmetic, bitwise, control flow, and stack manipulation instructions
- Written in pure Rust with minimal dependencies

## Opcodes

| Opcode              | Description                          |
|---------------------|--------------------------------------|
| `nop`               | No operation                         |
| `push`              | Push value onto the stack            |
| `pop`               | Pop value from the stack             |
| `plus` / `minus`    | Arithmetic addition / subtraction    |
| `swap`              | Swap top two stack values            |
| `copy`              | Duplicate top of stack               |
| `compare`           | Compare top two values               |
| `Do` / `Do_IF`      | Unconditional / conditional jump     |
| `obo` (XOR/AND/OR/NOT) | Bitwise operations                |
| `Done`              | Halt execution                       |
| `Duplicate_Select`  | Duplicate selected stack value       |
| `swap_Select`       | Swap selected values                 |
| `call` / `ret`      | Function call and return             |

> Note: The exact binary encoding of each opcode is defined in the tokenizer.

## Requirements

- Rust (edition 2024 or later)
- Cargo

## Build & Run

```bash
cd 3.6.0   # or whatever the current version folder is
cargo build --release
cargo run --release
```

When the program starts, enter a binary string (only `0` and `1`) and press Enter.

Example:
```
01011011...
```

## Project Structure

```
Backpack/
├── src/
│   ├── main.rs          # Entry point
│   ├── TokenCreate.rs   # Tokenizer
│   ├── Process.rs       # Instruction execution
│   └── Functions.rs     # Helper functions
├── Cargo.toml
└── LICENSE
```

## License

This project is licensed under the Apache License 2.0.  
See the [LICENSE](LICENSE) file for details.

## Author

BRONX,
