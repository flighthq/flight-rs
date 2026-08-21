import { mkdirSync } from 'node:fs';
import path from 'node:path';

import { portConfig } from '../port.config.ts';
import { analyzeUpstream } from './analyze/inventory.ts';
import { assertCompleteLoweringAudit, auditLowering } from './analyze/lowering.ts';
import { generateRust } from './emit/core.ts';
import {
  createApiReport,
  generationSummary,
  inventorySummary,
  loweringSummary,
  stableJson,
  writeOrCheck,
} from './emit/reports.ts';
import { assertBitmapWasmArtifactFresh } from './wasm-artifact.ts';

const argumentsSet = new Set(process.argv.slice(2));
const check = argumentsSet.has('--check');
const inventoryOnly = argumentsSet.has('--inventory');
const jsonOnly = argumentsSet.has('--json');
const workspaceDirectory = process.cwd();
const reportsDirectory = path.join(workspaceDirectory, portConfig.reportsDirectory);

try {
  const inventory = analyzeUpstream(workspaceDirectory);
  if (inventoryOnly && jsonOnly) {
    process.stdout.write(stableJson(inventory));
  } else {
    const api = createApiReport(inventory);
    const reports = [
      { content: stableJson(api), file: 'api.json' },
      { content: stableJson(inventory), file: 'inventory.json' },
      { content: inventorySummary(inventory), file: 'inventory.md' },
    ];

    if (!inventoryOnly) {
      const lowering = auditLowering(workspaceDirectory);
      // This is the exhaustive coverage gate. Keep it in the unbounded generator
      // check instead of duplicating the full upstream traversal in a timed unit test.
      if (check) assertCompleteLoweringAudit(lowering);
      const generation = generateRust(workspaceDirectory, check, inventory);
      reports.push(
        { content: stableJson(lowering), file: 'lowering.json' },
        { content: loweringSummary(lowering), file: 'lowering.md' },
        { content: stableJson(generation), file: 'generation.json' },
        { content: generationSummary(generation), file: 'generation.md' },
      );
      if (!check) mkdirSync(reportsDirectory, { recursive: true });
      for (const report of reports) {
        writeOrCheck(path.join(reportsDirectory, report.file), report.content, check);
      }
      if (check) assertBitmapWasmArtifactFresh(workspaceDirectory);
      const emitted = generation.targets.reduce((total, target) => total + target.emittedSources.length, 0);
      const excluded = generation.targets.reduce((total, target) => total + target.sourceExclusions.length, 0);
      const unsupported = generation.targets.reduce((total, target) => total + target.unsupportedSources.length, 0);
      process.stdout.write(
        `${check ? 'Verified' : 'Generated'} ${inventory.summary.packages} inventoried packages; emitted ${String(emitted)} Rust modules, with ${String(excluded)} explicit exclusions and ${String(unsupported)} unsupported sources.\n`,
      );
    } else {
      if (!check) mkdirSync(reportsDirectory, { recursive: true });
      for (const report of reports) {
        writeOrCheck(path.join(reportsDirectory, report.file), report.content, check);
      }
      process.stdout.write(
        `${check ? 'Verified' : 'Inventoried'} ${inventory.summary.packages} packages, ${inventory.summary.exportLanes} public lanes, ${inventory.summary.exports} export records, and ${inventory.summary.testFiles} tests.\n`,
      );
    }
  }
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(`${message}\n`);
  process.exitCode = 1;
}
