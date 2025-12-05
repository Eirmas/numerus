# Numerus++ for VS Code

Syntax highlighting for the **Numerus++** Roman-themed programming language.

## Features

- Syntax highlighting for `.npp` files
- Keywords: `DECLARA`, `EST`, `SCRIBE`, `AVTEM`
- Operators: `ADDIUS`, `SUBTRAHE`, `MULTIPLICA`, `DIVIDE`
- Built-in functions: `ROMANIZA`, `EXPRIME`
- Roman numeral literals (II, XIV, XLII, etc.)
- Arabic numeral literals
- String literals with escape sequences
- Comments with `NOTA:`
- Auto-closing brackets and quotes

## Installation

### Option 1: Copy to VS Code extensions folder

```bash
# Linux/macOS
cp -r vscode-numerus ~/.vscode/extensions/numerus-addiusaddius

# Windows
xcopy /E vscode-numerus %USERPROFILE%\.vscode\extensions\numerus-addiusaddius
```

Then restart VS Code.

### Option 2: Install via VSIX (recommended for sharing)

1. Install vsce if you haven't:
   ```bash
   npm install -g @vscode/vsce
   ```

2. Package the extension:
   ```bash
   cd vscode-numerus
   vsce package
   ```

3. Install the generated `.vsix` file:
   ```bash
   code --install-extension numerus-addiusaddius-0.1.0.vsix
   ```

### Option 3: Development mode

1. Open VS Code
2. Press `F5` or go to `Run > Start Debugging`
3. Select "VS Code Extension Development"
4. A new VS Code window will open with the extension loaded

## Language Example

```numerus
NOTA: Calculate the answer to everything
DECLARA universum EST XLII
DECLARA duo EST II

DECLARA responsum EST universum MULTIPLICA duo
SCRIBE("The answer times two: " ADDIUS responsum)
SCRIBE("In Arabic: " ADDIUS ARABIZA(responsum))

AVTEM
```

## Color Themes

The extension uses standard TextMate scopes that work with any VS Code theme:

| Element | Scope | Typical Color |
|---------|-------|---------------|
| Keywords | `keyword.control` | Purple/Blue |
| Operators | `keyword.operator` | Purple/Blue |
| Functions | `support.function` | Yellow/Cyan |
| Strings | `string.quoted` | Green/Orange |
| Numbers | `constant.numeric` | Orange/Cyan |
| Comments | `comment.line` | Gray/Green |
| Variables | `variable.other` | White/Light Blue |

## Contributing

Found a bug? Want to add a feature? PRs welcome!

## License

MIT

---

*GLORIA ROMAE IN PERPETUUM!*
