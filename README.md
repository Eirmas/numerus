# Numerus++

A Roman-themed mini programming language written in Rust.

## Quick Start

### Build

```bash
cargo build --release
```

### Run a Program

```bash
./target/release/numerus examples/basic.npp
```

### Start the REPL

```bash
./target/release/numerus
```

### Run Tests

```bash
cargo test
```

### Check Syntax (JSON output)

```bash
./target/release/numerus --check file.npp
```

## VS Code / Cursor Extension

The `vscode-numerus` folder contains a syntax highlighting extension.

### Install Extension

```bash
cd vscode-numerus
npm install
npm run build
vsce package
```

Then install the generated `.vsix` file via "Extensions: Install from VSIX" or copy to your extensions folder:

```
~/.cursor/extensions/aritma.numerus-addiusaddius-0.3.0/
```

## Language Reference

### Variables

```
DECLARA name EST value       NOTA: Declare a variable
name EST newValue            NOTA: Reassign a variable
```

### Data Types

- **Numbers**: Arabic (`42`) or Roman (`XLII`)
- **Strings**: `"Hello World"`

### Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `ADDIUS` | Add / Concatenate | `10 ADDIUS 5` or `"Hi " ADDIUS name` |
| `SUBTRAHE` | Subtract | `10 SUBTRAHE 3` |
| `MULTIPLICA` | Multiply | `6 MULTIPLICA 7` |
| `DIVIDE` | Divide | `42 DIVIDE 6` |

Precedence: `MULTIPLICA` and `DIVIDE` bind tighter than `ADDIUS` and `SUBTRAHE`.

### Output

```
SCRIBE(expression)           NOTA: Print (numbers display as Roman numerals)
```

### Built-in Functions

```
ROMANIZA(number)             NOTA: Convert number to Roman string
ARABIZA(number)              NOTA: Convert number to Arabic string
```

### Comments

```
NOTA: This is a comment
```

### No-Op

```
AVTEM                        NOTA: Does nothing, but with Roman gravitas
```

## Example

```
NOTA: Calculate and display results
DECLARA x EST 10
DECLARA y EST XXXII

DECLARA sum EST x ADDIUS y
SCRIBE("Sum: " ADDIUS sum)
SCRIBE("In Arabic: " ADDIUS ARABIZA(sum))

AVTEM
```

Output:
```
Sum: XLII
In Arabic: 42
```

---

*GLORIA ROMAE IN PERPETUUM!*
