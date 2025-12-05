import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

let diagnosticCollection: vscode.DiagnosticCollection;

// Hover documentation for Numerus++ keywords
const hoverDocs: { [key: string]: { title: string; description: string; example?: string } } = {
    'DECLARA': {
        title: 'DECLARA - Variable Declaration',
        description: 'Declares a new variable with an initial value. Variables can hold numbers (Arabic or Roman) or strings.',
        example: 'DECLARA myVar EST 42\nDECLARA greeting EST "Salve!"'
    },
    'EST': {
        title: 'EST - Assignment Operator',
        description: 'Assigns a value to a variable. Used both in declarations (with DECLARA) and reassignments.',
        example: 'DECLARA X EST 10\nX EST X ADDIUS 5'
    },
    'ADDIUS': {
        title: 'ADDIUS - Addition / Concatenation',
        description: 'Adds two numbers together, or concatenates strings. When mixing strings and numbers, numbers are converted to Roman numerals.',
        example: 'DECLARA sum EST 10 ADDIUS 5\nDECLARA msg EST "Value: " ADDIUS sum'
    },
    'SUBTRAHE': {
        title: 'SUBTRAHE - Subtraction',
        description: 'Subtracts the second number from the first. Only works with numbers.',
        example: 'DECLARA diff EST 10 SUBTRAHE 3'
    },
    'MULTIPLICA': {
        title: 'MULTIPLICA - Multiplication',
        description: 'Multiplies two numbers together. Has higher precedence than ADDIUS and SUBTRAHE.',
        example: 'DECLARA product EST 6 MULTIPLICA 7'
    },
    'DIVIDE': {
        title: 'DIVIDE - Division',
        description: 'Divides the first number by the second (integer division). Division by zero is forbidden!',
        example: 'DECLARA quotient EST 42 DIVIDE 6'
    },
    'SCRIBE': {
        title: 'SCRIBE - Print Output',
        description: 'Prints a value to the console. Numbers are displayed as Roman numerals by default. Use ARABIZA() to convert numbers to Arabic format.',
        example: 'SCRIBE("Hello World")\nSCRIBE(myVar)\nSCRIBE(ARABIZA(myVar))'
    },
    'ARABIZA': {
        title: 'ARABIZA - Convert to Arabic String',
        description: 'Built-in function that converts a number to its Arabic (decimal) string representation. Useful for displaying numbers as Arabic instead of Roman.',
        example: 'SCRIBE(ARABIZA(42))  NOTA: Outputs "42"\nDECLARA str EST ARABIZA(100)  NOTA: str = "100"'
    },
    'ROMANIZA': {
        title: 'ROMANIZA - Convert to Roman String',
        description: 'Built-in function that converts a number to its Roman numeral string representation.',
        example: 'DECLARA roman EST ROMANIZA(42)\nNOTA: roman now contains "XLII"'
    },
    'AVTEM': {
        title: 'AVTEM - Ceremonial No-Op',
        description: 'Does absolutely nothing, but adds tremendous Roman gravitas to your code. Use liberally for dramatic effect.',
        example: 'AVTEM\nAVTEM\nAVTEM'
    },
    'NOTA': {
        title: 'NOTA: - Comment',
        description: 'Everything after NOTA: on a line is a comment and will be ignored by the interpreter.',
        example: 'NOTA: This is a comment\nDECLARA X EST 42  NOTA: inline comment'
    }
};

export function activate(context: vscode.ExtensionContext) {
    console.log('Numerus++ extension activated!');

    diagnosticCollection = vscode.languages.createDiagnosticCollection('numerus');
    context.subscriptions.push(diagnosticCollection);

    // Register hover provider
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('numerus', {
            provideHover(document, position, _token) {
                const range = document.getWordRangeAtPosition(position);
                if (!range) {
                    return null;
                }

                const word = document.getText(range).toUpperCase();
                const doc = hoverDocs[word];

                if (doc) {
                    const markdown = new vscode.MarkdownString();
                    markdown.appendMarkdown(`### ${doc.title}\n\n`);
                    markdown.appendMarkdown(`${doc.description}\n\n`);
                    if (doc.example) {
                        markdown.appendMarkdown(`**Example:**\n\`\`\`numerus\n${doc.example}\n\`\`\``);
                    }
                    return new vscode.Hover(markdown, range);
                }

                return null;
            }
        })
    );

    // Check on file open
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(doc => {
            if (doc.languageId === 'numerus') {
                checkDocument(doc);
            }
        })
    );

    // Check on file save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(doc => {
            if (doc.languageId === 'numerus') {
                checkDocument(doc);
            }
        })
    );

    // Check on file change (with debounce)
    let timeout: NodeJS.Timeout | undefined;
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            if (event.document.languageId === 'numerus') {
                if (timeout) {
                    clearTimeout(timeout);
                }
                timeout = setTimeout(() => {
                    checkDocument(event.document);
                }, 500);
            }
        })
    );

    // Check all open numerus files on activation
    vscode.workspace.textDocuments.forEach(doc => {
        if (doc.languageId === 'numerus') {
            checkDocument(doc);
        }
    });
}

function checkDocument(document: vscode.TextDocument) {
    const config = vscode.workspace.getConfiguration('numerus');
    const configuredPath = config.get<string>('executablePath') || '';

    // Possible paths to try
    const possiblePaths = configuredPath ? [configuredPath] : [
        '/root/aritma/coding-dojo/numerus/target/release/numerus',
        path.join(os.homedir(), '.cargo/bin/numerus'),
        'numerus',
    ];

    // Write content to temp file
    const tmpFile = path.join(os.tmpdir(), `numerus_check_${Date.now()}.npp`);

    try {
        fs.writeFileSync(tmpFile, document.getText());
    } catch (err) {
        console.error('Numerus++: Failed to write temp file:', err);
        return;
    }

    tryCheckWithPaths(possiblePaths, tmpFile, document, () => {
        try { fs.unlinkSync(tmpFile); } catch (e) { /* ignore */ }
    });
}

function tryCheckWithPaths(
    paths: string[],
    tmpFile: string,
    document: vscode.TextDocument,
    cleanup: () => void
) {
    if (paths.length === 0) {
        console.error('Numerus++: Could not find numerus executable');
        cleanup();
        return;
    }

    const numerusPath = paths[0];
    const remainingPaths = paths.slice(1);

    cp.exec(
        `"${numerusPath}" --check "${tmpFile}"`,
        { timeout: 5000 },
        (error, stdout, _stderr) => {
            // If command not found, try next path
            if (error && (error.code === 127 || (error.message && error.message.includes('ENOENT')))) {
                tryCheckWithPaths(remainingPaths, tmpFile, document, cleanup);
                return;
            }

            cleanup();

            try {
                const result = JSON.parse(stdout);
                const diagnostics: vscode.Diagnostic[] = [];

                for (const diag of result.diagnostics || []) {
                    const range = new vscode.Range(
                        Math.max(0, diag.line - 1),
                        Math.max(0, diag.column - 1),
                        Math.max(0, diag.end_line - 1),
                        Math.max(0, diag.end_column - 1)
                    );

                    const severity = diag.severity === 'error'
                        ? vscode.DiagnosticSeverity.Error
                        : diag.severity === 'warning'
                        ? vscode.DiagnosticSeverity.Warning
                        : vscode.DiagnosticSeverity.Information;

                    const diagnostic = new vscode.Diagnostic(range, diag.message, severity);
                    diagnostic.source = 'numerus';
                    diagnostics.push(diagnostic);
                }

                diagnosticCollection.set(document.uri, diagnostics);
            } catch (parseError) {
                console.error('Numerus++: Failed to parse output:', stdout, parseError);
            }
        }
    );
}

export function deactivate() {
    if (diagnosticCollection) {
        diagnosticCollection.dispose();
    }
}
