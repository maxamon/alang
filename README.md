# 🚀 Alang - A Lisp-like Programming Language

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

**A minimal, elegant Lisp-inspired language with a bytecode VM**

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Syntax](#-syntax) • [Examples](#-examples)

</div>

---

## 📖 Overview

**Alang** is a lightweight, Lisp-like programming language implemented in Rust. It features a complete interpreter toolchain including:

- 🔍 **Parser** - S-expression parser for Lisp-like syntax
- 🔨 **Compiler** - Compiles to efficient bytecode
- ⚡ **Virtual Machine** - Stack-based bytecode interpreter with closure support

Alang is designed for educational purposes and demonstrates the implementation of a functional programming language with lexical scoping, first-class functions, and lambda calculus.

## ✨ Features

- **✅ Functional Programming**: First-class functions and lambda expressions
- **✅ Lexical Scoping**: Proper variable scoping with closures
- **✅ Conditionals**: If-then-else expressions
- **✅ Local Bindings**: Let expressions for local variables
- **✅ Global Definitions**: Define global variables and functions
- **✅ Arithmetic**: Basic arithmetic operations
- **✅ Sequential Execution**: Begin blocks for multiple expressions
- **✅ Bytecode Compilation**: Compiles to optimized bytecode
- **✅ Stack-based VM**: Efficient virtual machine execution

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/maxamon/alang.git
cd alang

# Build the project
cargo build --release

# The binary will be available at target/release/Alang
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## 🎯 Usage

### Running a Program

```bash
# Run a program from a file
cargo run --release <filename.alang>

# Or use the compiled binary
./target/release/Alang <filename.alang>
```

### Example

```bash
# Run the included test program
cargo run --release test.alang
```

## 📝 Syntax

Alang uses S-expressions (symbolic expressions) similar to Lisp and Scheme. All expressions are enclosed in parentheses.

### Basic Syntax Rules

- **Numbers**: Integer literals (e.g., `42`, `-10`)
- **Variables**: Alphanumeric identifiers (e.g., `x`, `myVar`, `count`)
- **Expressions**: All operations use prefix notation: `(operator operand1 operand2 ...)`

## 💡 Examples

### 1. Numbers

```lisp
42
-10
0
```

### 2. Arithmetic Operations

```lisp
; Addition
(+ 5 3)          ; Returns: 8

; Nested arithmetic
(+ 10 (+ 5 3))   ; Returns: 18

; Negative numbers
(+ -5 10)        ; Returns: 5
```

### 3. Variables with Let

The `let` expression binds a value to a variable in a local scope:

```lisp
; Simple let binding
(let x 10
  (+ x 5))       ; Returns: 15

; Nested let bindings
(let a 5
  (let b 10
    (+ a b)))    ; Returns: 15

; Let with multiple expressions in body
(let x 10
  (let y 20
    (+ x y)))    ; Returns: 30
```

### 4. Lambda Functions

Lambda expressions create anonymous functions:

```lisp
; Simple lambda
(lambda x (+ x 1))

; Lambda with application
((lambda x (+ x 1)) 5)   ; Returns: 6

; Lambda that doubles a number
((lambda n (+ n n)) 10)  ; Returns: 20
```

### 5. Function Application

```lisp
; Applying a lambda function
((lambda x (+ x 10)) 5)  ; Returns: 15

; Nested function application
((lambda x ((lambda y (+ x y)) 3)) 5)  ; Returns: 8
```

### 6. Let with Functions

```lisp
; Bind a lambda to a variable
(let add1 (lambda x (+ x 1))
  (add1 5))      ; Returns: 6

; More complex example
(let double (lambda n (+ n n))
  (double 21))   ; Returns: 42
```

### 7. Conditional Expressions (If)

The `if` expression evaluates a condition and returns one of two values:

```lisp
; Basic if expression (0 is false, non-zero is true)
(if 1 10 20)     ; Returns: 10
(if 0 10 20)     ; Returns: 20

; If with variables
(let x 5
  (if x 
    (+ x 10)     ; Returns: 15 (because x is non-zero)
    (+ x 100)))

; If with negative numbers
(if -5 10 20)    ; Returns: 10 (non-zero is true)
```

### 8. Complex Example - Conditional with Lambda

```lisp
; Lambda that checks if number is non-zero
((lambda x 
   (if x 
     (+ 2 x)     ; If x is non-zero, add 2
     100))       ; Otherwise return 100
 5)              ; Returns: 7

; Same lambda with zero
((lambda x 
   (if x 
     (+ 2 x) 
     100)) 
 0)              ; Returns: 100
```

### 9. Global Definitions with Begin

The `begin` block allows multiple expressions to be evaluated in sequence, with support for `define` to create global variables:

```lisp
(begin
  (define x 10)
  (define y (lambda z (+ 2 z)))
  (define v (y (y x)))
  (if (+ -10 v) 
    (+ 1 v) 
    (let z (+ 2 x) z)))
; This evaluates all definitions and returns the final expression
```

### 10. Complete Program Example

Here's a complete example that demonstrates multiple features:

```lisp
(begin
  ; Define a global variable
  (define x 10)
  
  ; Define a function that adds 2 to its argument
  (define add2 (lambda z (+ 2 z)))
  
  ; Apply the function twice
  (define result (add2 (add2 x)))
  
  ; Conditional based on result
  (if (+ -10 result)
    (+ 1 result)
    (let temp (+ 2 x) temp)))
; Returns: 15
```

## 🏗️ Architecture

### Parser (`parser.rs`)

Parses S-expressions into an Abstract Syntax Tree (AST):
- Handles numbers, variables, and compound expressions
- Supports keywords: `lambda`, `let`, `if`, `define`, `begin`
- Produces an `Expr` enum representing the AST

### Compiler (`compiler.rs`)

Compiles AST to bytecode:
- Generates stack-based bytecode operations
- Handles closure conversion and free variable capture
- Manages constant pool and local/global variables
- Outputs `Op` (operations) and `Value` (constants)

### Virtual Machine (`vm.rs`)

Executes compiled bytecode:
- Stack-based architecture
- Frame-based function calls with proper scoping
- Supports closures with captured variables
- Operations include: PushConst, PushLocal, Call, Ret, Add, Jump, etc.

### Bytecode Operations

The VM supports the following operations:

- `PushConst(idx)` - Push constant from pool
- `PushLocal(idx)` - Push local variable
- `PushGlobal(idx)` - Push global variable
- `StoreLocal(idx)` - Store to local variable
- `StoreGlobal(idx)` - Store to global variable
- `Call(argc)` - Call function with arguments
- `Ret` - Return from function
- `Add` - Add two numbers
- `JumpIfFalse(offset)` - Conditional jump
- `Jump(offset)` - Unconditional jump
- `MakeClosure(num_free)` - Create closure
- `Pop` - Pop value from stack

## 🧪 Running Tests

The project includes comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test parse_let_test

# Run with verbose output
cargo test -- --nocapture
```

## 🏃 Development

### Project Structure

```
alang/
├── src/
│   ├── main.rs       # Entry point and main program logic
│   ├── parser.rs     # S-expression parser
│   ├── compiler.rs   # Bytecode compiler
│   └── vm.rs         # Virtual machine
├── tests/
│   └── integration_test.rs
├── test.alang        # Example program
├── Cargo.toml        # Project configuration
└── README.md         # This file
```

### Building for Development

```bash
# Build in debug mode
cargo build

# Run with debug output
cargo run test.alang

# Check for errors without building
cargo check
```

## 📚 Language Reference

### Expression Types

| Expression | Syntax | Description |
|------------|--------|-------------|
| Number | `42` | Integer literal |
| Variable | `x` | Variable reference |
| Addition | `(+ a b)` | Add two expressions |
| Lambda | `(lambda x body)` | Function definition |
| Application | `(func arg)` | Function call |
| Let | `(let var val body)` | Local binding |
| If | `(if cond then else)` | Conditional |
| Define | `(define var val)` | Global definition |
| Begin | `(begin expr...)` | Sequential execution |

### Truth Values

- **Truthy**: Any non-zero integer
- **Falsy**: Zero (`0`)

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Write tests for new features
- Follow Rust coding conventions
- Update documentation for new features
- Ensure all tests pass before submitting

## 📄 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

- Inspired by Scheme, Lisp, and other functional programming languages
- Built with Rust for safety and performance
- Educational project demonstrating compiler construction

## 📮 Contact

For questions or suggestions, please open an issue on GitHub.

---

<div align="center">

Made with ❤️ using Rust

[⬆ Back to Top](#-alang---a-lisp-like-programming-language)

</div>
