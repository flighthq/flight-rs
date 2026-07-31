import { createHash } from 'node:crypto';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import type { ConformanceHarvestPackage } from '../../port.config.ts';
import type { PackageInventory } from '../model/inventory.ts';

export interface ConformanceUnsupportedTest {
  line?: number;
  reason: string;
  test?: string;
}

export interface ConformanceFileReport {
  fingerprint: string;
  source: string;
  status: 'partial' | 'translated' | 'unsupported';
  testCases: number;
  translatedCases: number;
  unsupported: ConformanceUnsupportedTest[];
}

export interface ConformancePackageReport {
  crate: string;
  package: string;
  passingCases: number;
  passingTestFiles: number;
  testFiles: ConformanceFileReport[];
  translatedCases: number;
  translatedTestFiles: number;
  unsupportedTestFiles: number;
}

export interface ConformanceHarvestReport {
  packages: ConformancePackageReport[];
  summary: {
    inScopeTestFiles: number;
    outOfScopeTestFiles: number;
    passingCases: number;
    passingTestFiles: number;
    totalUpstreamTestFiles: number;
    translatedCases: number;
    translatedTestFiles: number;
    unsupportedTestFiles: number;
  };
}

export interface ConformanceHarvestOutput {
  report: ConformanceHarvestReport;
  rustModules: ReadonlyMap<string, string>;
}

interface ImportedBinding {
  parameters?: Array<{ optional: boolean }>;
  rust: string;
}

interface TestCase {
  body?: ts.Block;
  line: number;
  name: string;
  suite: string[];
}

interface TranslatedTest {
  rust: string;
  test: TestCase;
}

export function harvestConformance(
  workspaceDirectory: string,
  packages: readonly PackageInventory[],
  configuration: readonly ConformanceHarvestPackage[],
  totalUpstreamTestFiles: number,
): ConformanceHarvestOutput {
  const rustModules = new Map<string, string>();
  const packageReports = configuration.map((selection): ConformancePackageReport => {
    const packageInventory = packages.find((item) => item.name === selection.package);
    if (!packageInventory) throw new Error(`Conformance harvest package is absent: ${selection.package}`);
    const sourceDirectory = path.join(workspaceDirectory, packageInventory.directory, 'src');
    const selected = new Set(selection.sources);
    const stale = new Set(selection.sources);
    const translatedRust: string[] = [];
    const testFiles = walkTestFiles(sourceDirectory).map((file): ConformanceFileReport => {
      const relativeSource = path.relative(sourceDirectory, file).split(path.sep).join('/');
      const source = path.relative(workspaceDirectory, file).split(path.sep).join('/');
      const sourceText = readFileSync(file, 'utf8');
      const fingerprint = sha256(sourceText);
      const sourceFile = ts.createSourceFile(file, sourceText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const tests = collectTests(sourceFile);
      if (!selected.has(relativeSource)) {
        return {
          fingerprint,
          source,
          status: 'unsupported',
          testCases: tests.length,
          translatedCases: 0,
          unsupported: [
            {
              reason: selection.unsupportedReason,
            },
          ],
        };
      }
      stale.delete(relativeSource);
      const imports = collectImportedBindings(sourceFile, file);
      const translated: TranslatedTest[] = [];
      const unsupported: ConformanceUnsupportedTest[] = [];
      for (const [index, test] of tests.entries()) {
        try {
          translated.push(translateTest(test, imports, relativeSource, index));
        } catch (error) {
          unsupported.push({
            line: test.line,
            reason: error instanceof Error ? error.message : String(error),
            test: [...test.suite, test.name].join(' > '),
          });
        }
      }
      translatedRust.push(...translated.map((item) => item.rust));
      return {
        fingerprint,
        source,
        status: unsupported.length === 0 && translated.length === tests.length ? 'translated' : 'partial',
        testCases: tests.length,
        translatedCases: translated.length,
        unsupported,
      };
    });
    if (stale.size > 0) {
      throw new Error(`Stale conformance source selections for ${selection.package}: ${[...stale].join(', ')}`);
    }
    if (translatedRust.length > 0) {
      rustModules.set(selection.package, emitRustModule(selection.package, translatedRust));
    }
    const translatedTestFiles = testFiles.filter((item) => item.status === 'translated').length;
    const translatedCases = testFiles.reduce((total, item) => total + item.translatedCases, 0);
    return {
      crate: packageInventory.rustCrate,
      package: packageInventory.name,
      passingCases: 0,
      passingTestFiles: 0,
      testFiles,
      translatedCases,
      translatedTestFiles,
      unsupportedTestFiles: testFiles.length - translatedTestFiles,
    };
  });
  const inScopeTestFiles = packageReports.reduce((total, item) => total + item.testFiles.length, 0);
  const translatedTestFiles = packageReports.reduce((total, item) => total + item.translatedTestFiles, 0);
  const translatedCases = packageReports.reduce((total, item) => total + item.translatedCases, 0);
  return {
    report: {
      packages: packageReports,
      summary: {
        inScopeTestFiles,
        outOfScopeTestFiles: totalUpstreamTestFiles - inScopeTestFiles,
        passingCases: 0,
        passingTestFiles: 0,
        totalUpstreamTestFiles,
        translatedCases,
        translatedTestFiles,
        unsupportedTestFiles: inScopeTestFiles - translatedTestFiles,
      },
    },
    rustModules,
  };
}

export function markConformancePassing(report: ConformanceHarvestReport): ConformanceHarvestReport {
  const packages = report.packages.map((item) => ({
    ...item,
    passingCases: item.translatedCases,
    passingTestFiles: item.translatedTestFiles,
  }));
  return {
    packages,
    summary: {
      ...report.summary,
      passingCases: report.summary.translatedCases,
      passingTestFiles: report.summary.translatedTestFiles,
    },
  };
}

function walkTestFiles(directory: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...walkTestFiles(file));
    } else if (entry.isFile() && (entry.name.endsWith('.test.ts') || entry.name.endsWith('.spec.ts'))) {
      files.push(file);
    }
  }
  return files.sort((left, right) => left.localeCompare(right));
}

function collectTests(sourceFile: ts.SourceFile): TestCase[] {
  const tests: TestCase[] = [];
  const visit = (statements: ts.NodeArray<ts.Statement>, suite: string[]): void => {
    for (const statement of statements) {
      if (!ts.isExpressionStatement(statement) || !ts.isCallExpression(statement.expression)) continue;
      const call = statement.expression;
      const name = callName(call.expression);
      if (name === 'describe') {
        const description = stringArgument(call.arguments[0]);
        const callback = call.arguments[1];
        if (description && callback && ts.isArrowFunction(callback) && ts.isBlock(callback.body)) {
          visit(callback.body.statements, [...suite, description]);
        }
        continue;
      }
      if (name !== 'it' && name !== 'test') continue;
      const description = stringArgument(call.arguments[0]) ?? '<dynamic test name>';
      const callback = call.arguments[1];
      const body = callback && ts.isArrowFunction(callback) && ts.isBlock(callback.body) ? callback.body : undefined;
      tests.push({
        ...(body ? { body } : {}),
        line: sourceFile.getLineAndCharacterOfPosition(call.getStart(sourceFile)).line + 1,
        name: description,
        suite,
      });
    }
  };
  visit(sourceFile.statements, []);
  return tests;
}

function collectImportedBindings(sourceFile: ts.SourceFile, testFile: string): ReadonlyMap<string, ImportedBinding> {
  const bindings = new Map<string, ImportedBinding>();
  for (const statement of sourceFile.statements) {
    if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
    if (!statement.moduleSpecifier.text.startsWith('.')) continue;
    const clause = statement.importClause?.namedBindings;
    if (!clause || !ts.isNamedImports(clause)) continue;
    const implementation = resolveImplementation(testFile, statement.moduleSpecifier.text);
    const signatures = implementation ? collectFunctionSignatures(implementation) : new Map();
    for (const element of clause.elements) {
      const imported = element.propertyName?.text ?? element.name.text;
      bindings.set(element.name.text, {
        rust: signatures.has(imported) ? snakeCase(imported) : screamingSnakeCase(imported),
        ...(signatures.get(imported) ? { parameters: signatures.get(imported) } : {}),
      });
    }
  }
  return bindings;
}

function resolveImplementation(testFile: string, moduleSpecifier: string): ts.SourceFile | undefined {
  const file = path.resolve(path.dirname(testFile), `${moduleSpecifier}.ts`);
  try {
    return ts.createSourceFile(file, readFileSync(file, 'utf8'), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  } catch {
    return undefined;
  }
}

function collectFunctionSignatures(sourceFile: ts.SourceFile): ReadonlyMap<string, Array<{ optional: boolean }>> {
  const signatures = new Map<string, Array<{ optional: boolean }>>();
  for (const statement of sourceFile.statements) {
    if (!ts.isFunctionDeclaration(statement) || !statement.name) continue;
    signatures.set(
      statement.name.text,
      statement.parameters.map((parameter) => ({
        optional: Boolean(parameter.initializer || parameter.questionToken),
      })),
    );
  }
  return signatures;
}

function translateTest(
  test: TestCase,
  imports: ReadonlyMap<string, ImportedBinding>,
  source: string,
  index: number,
): TranslatedTest {
  if (!test.body) throw new Error('test callback must be a block-bodied arrow function');
  const statements: string[] = [];
  for (const statement of test.body.statements) {
    if (ts.isVariableStatement(statement)) {
      if (statement.declarationList.declarations.length !== 1) {
        throw new Error('one variable declaration per statement is required');
      }
      const declaration = statement.declarationList.declarations[0]!;
      if (!ts.isIdentifier(declaration.name) || !declaration.initializer) {
        throw new Error('variable declarations require an identifier and initializer');
      }
      statements.push(`let ${snakeCase(declaration.name.text)} = ${emitExpression(declaration.initializer, imports)};`);
      continue;
    }
    if (ts.isExpressionStatement(statement)) {
      statements.push(emitAssertion(statement.expression, imports));
      continue;
    }
    throw new Error(`unsupported test statement: ${ts.SyntaxKind[statement.kind]}`);
  }
  const displayName = [...test.suite, test.name].join(' > ');
  const functionName = `upstream_${source.replace(/\.test\.ts$/u, '').replace(/\.spec\.ts$/u, '')}_${String(index + 1)}_${displayName}`;
  return {
    rust: [
      `// ${source}:${String(test.line)} — ${displayName}`,
      '#[test]',
      `fn ${safeRustIdentifier(functionName)}() {`,
      '  let _flight_task_scheduler = crate::install_deterministic_flight_task_scheduler();',
      ...statements.map((statement) => `  ${statement}`),
      '}',
    ].join('\n'),
    test,
  };
}

function emitAssertion(expression: ts.Expression, imports: ReadonlyMap<string, ImportedBinding>): string {
  if (!ts.isCallExpression(expression) || !ts.isPropertyAccessExpression(expression.expression)) {
    throw new Error('test statement must be an expect matcher');
  }
  const matcher = expression.expression.name.text;
  const expectation = expression.expression.expression;
  if (
    !ts.isCallExpression(expectation) ||
    !ts.isIdentifier(expectation.expression) ||
    expectation.expression.text !== 'expect'
  ) {
    throw new Error('test statement must call expect(...)');
  }
  const actualNode = expectation.arguments[0];
  if (!actualNode) throw new Error('expect(...) requires an actual value');
  const actual = emitExpression(actualNode, imports);
  if (matcher === 'toBe') {
    const expected = expression.arguments[0];
    if (!expected) throw new Error('toBe requires an expected value');
    return `assert_eq!(${actual}, ${emitExpression(expected, imports)});`;
  }
  if (matcher === 'toBeNaN') return `assert!(${parenthesize(actual)}.is_nan());`;
  if (matcher === 'toBeCloseTo') {
    const expected = expression.arguments[0];
    const precision = expression.arguments[1];
    if (!expected || !precision || !ts.isNumericLiteral(precision)) {
      throw new Error('toBeCloseTo requires an expected value and literal precision');
    }
    return `flight_close(${actual}, ${emitExpression(expected, imports)}, ${precision.text}_i32);`;
  }
  if (matcher === 'toBeGreaterThan' || matcher === 'toBeLessThan') {
    const expected = expression.arguments[0];
    if (!expected) throw new Error(`${matcher} requires an expected value`);
    const operator = matcher === 'toBeGreaterThan' ? '>' : '<';
    return `assert!(${actual} ${operator} ${emitExpression(expected, imports)});`;
  }
  throw new Error(`unsupported expect matcher: ${matcher}`);
}

function emitExpression(expression: ts.Expression, imports: ReadonlyMap<string, ImportedBinding>): string {
  if (ts.isParenthesizedExpression(expression)) return parenthesize(emitExpression(expression.expression, imports));
  if (ts.isNumericLiteral(expression)) return rustNumber(Number(expression.text));
  if (expression.kind === ts.SyntaxKind.TrueKeyword) return 'true';
  if (expression.kind === ts.SyntaxKind.FalseKeyword) return 'false';
  if (ts.isIdentifier(expression)) {
    if (expression.text === 'NaN') return 'f64::NAN';
    const imported = imports.get(expression.text);
    return imported ? `crate::${imported.rust}` : snakeCase(expression.text);
  }
  if (ts.isPrefixUnaryExpression(expression)) {
    const operand = emitExpression(expression.operand, imports);
    if (expression.operator === ts.SyntaxKind.MinusToken) return `-${parenthesize(operand)}`;
    if (expression.operator === ts.SyntaxKind.PlusToken) return operand;
    throw new Error(`unsupported unary operator: ${ts.SyntaxKind[expression.operator]}`);
  }
  if (ts.isBinaryExpression(expression)) {
    const operator = binaryOperator(expression.operatorToken.kind);
    return `(${emitExpression(expression.left, imports)} ${operator} ${emitExpression(expression.right, imports)})`;
  }
  if (ts.isCallExpression(expression)) {
    if (!ts.isIdentifier(expression.expression)) throw new Error('only direct function calls are supported');
    const binding = imports.get(expression.expression.text);
    const callee = binding ? `crate::${binding.rust}` : snakeCase(expression.expression.text);
    const parameters = binding?.parameters ?? [];
    const arguments_ = expression.arguments.map((argument, index) => {
      const emitted = emitExpression(argument, imports);
      return parameters[index]?.optional ? `Some(${emitted})` : emitted;
    });
    for (let index = arguments_.length; index < parameters.length; index++) {
      if (!parameters[index]?.optional)
        throw new Error(`call to ${expression.expression.text} omits a required argument`);
      arguments_.push('None');
    }
    return `${callee}(${arguments_.join(', ')})`;
  }
  if (
    ts.isPropertyAccessExpression(expression) &&
    ts.isIdentifier(expression.expression) &&
    expression.expression.text === 'Math' &&
    expression.name.text === 'PI'
  ) {
    return 'std::f64::consts::PI';
  }
  throw new Error(`unsupported test expression: ${ts.SyntaxKind[expression.kind]}`);
}

function emitRustModule(packageName: string, tests: readonly string[]): string {
  return [
    `// @generated from upstream ${packageName} tests; do not edit.`,
    '',
    'fn flight_close(actual: f64, expected: f64, precision: i32) {',
    '  let tolerance = 0.5_f64 * 10.0_f64.powi(-precision);',
    '  assert!((actual - expected).abs() <= tolerance, "actual={actual}, expected={expected}, precision={precision}");',
    '}',
    '',
    ...tests,
    '',
  ].join('\n');
}

function callName(expression: ts.LeftHandSideExpression): string | undefined {
  if (ts.isIdentifier(expression)) return expression.text;
  if (ts.isPropertyAccessExpression(expression) && ts.isIdentifier(expression.expression)) {
    return expression.expression.text;
  }
  return undefined;
}

function stringArgument(expression: ts.Expression | undefined): string | undefined {
  return expression && (ts.isStringLiteral(expression) || ts.isNoSubstitutionTemplateLiteral(expression))
    ? expression.text
    : undefined;
}

function binaryOperator(kind: ts.SyntaxKind): string {
  switch (kind) {
    case ts.SyntaxKind.PlusToken:
      return '+';
    case ts.SyntaxKind.MinusToken:
      return '-';
    case ts.SyntaxKind.AsteriskToken:
      return '*';
    case ts.SyntaxKind.SlashToken:
      return '/';
    default:
      throw new Error(`unsupported binary operator: ${ts.SyntaxKind[kind]}`);
  }
}

function rustNumber(value: number): string {
  if (!Number.isFinite(value)) throw new Error(`non-finite numeric literal is unsupported: ${String(value)}`);
  return `${Number.isInteger(value) ? `${String(value)}.0` : String(value)}_f64`;
}

function snakeCase(value: string): string {
  return value
    .replace(/([a-z\d])([A-Z])/gu, '$1_$2')
    .replace(/[^A-Za-z0-9_]+/gu, '_')
    .toLowerCase();
}

function screamingSnakeCase(value: string): string {
  return snakeCase(value).toUpperCase();
}

function safeRustIdentifier(value: string): string {
  const normalized = snakeCase(value).replace(/^\d/u, '_$&').replace(/_+/gu, '_').replace(/^_|_$/gu, '');
  return normalized.length > 0 ? normalized : 'upstream_test';
}

function parenthesize(value: string): string {
  return `(${value})`;
}

function sha256(value: string): string {
  return createHash('sha256').update(value).digest('hex');
}
