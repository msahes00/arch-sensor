# arch-sensor
An assembly architecture detection tool


## Overview

`arch-sensor` is a Rust-based tool for detecting assembly language architectures based on mnemonic patterns. It analyzes a given input (file or direct input via command line) and matches its content against predefined sets of mnemonics associated with various architectures.

## Documentation
For the [obsidian](https://obsidian.md) docs about the project, [see here](docs/index.md)

## Supported Architectures

The following architectures are currently supported by `arch-sensor`:
- ARM
- AVR
- Intel
- Moxie
- PowerPC
- RISC-V
- SPARC
- W6502

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/arch-sensor.git
   cd arch-sensor
   ```
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## Usage

### Command Line

You can provide input to `arch-sensor` in one of two ways:

1. **From a file**:
   ```bash
   ./arch-sensor path/to/assembly-code.s
   ```
2. **Directly via command-line input**:
   ```bash
   ./arch-sensor - "MOV R0, R1; ADD R2, R3"
   ```

### Example Output

After running the tool, you will see something like:
```
Detected Architecture Mnemonic Counts:
--------------------------------------
arm: 3
riscv: 1
intel: 2
...
--------------------------------------
The detected architecture is 'arm' (most likely).
```

## Special Mentions

A special mention goes to the [Compiler-Explorer Infraestructure repository](https://github.com/compiler-explorer/infra), which has been a helpful tool for testing arch-sensor across multiple architectures.

## License

This project is licensed under the terms of the [MIT License](LICENSE).