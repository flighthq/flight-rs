import { mkdirSync } from 'node:fs';
import path from 'node:path';

import { portConfig } from '../port.config.ts';
import { analyzeUpstream } from './analyze/inventory.ts';
import { auditLowering } from './analyze/lowering.ts';
import { generateRust } from './emit/core.ts';
import {
  createApiReport,
  generationSummary,
  inventorySummary,
  loweringSummary,
  stableJson,
  writeOrCheck,
} from './emit/reports.ts';

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
    if (!check) mkdirSync(reportsDirectory, { recursive: true });
    const api = createApiReport(inventory);
    writeOrCheck(path.join(reportsDirectory, 'api.json'), stableJson(api), check);
    writeOrCheck(path.join(reportsDirectory, 'inventory.json'), stableJson(inventory), check);
    writeOrCheck(path.join(reportsDirectory, 'inventory.md'), inventorySummary(inventory), check);

    if (!inventoryOnly) {
      const lowering = auditLowering(workspaceDirectory);
      const generation = generateRust(workspaceDirectory, check, inventory);
      writeOrCheck(path.join(reportsDirectory, 'lowering.json'), stableJson(lowering), check);
      writeOrCheck(path.join(reportsDirectory, 'lowering.md'), loweringSummary(lowering), check);
      writeOrCheck(path.join(reportsDirectory, 'generation.json'), stableJson(generation), check);
      writeOrCheck(path.join(reportsDirectory, 'generation.md'), generationSummary(generation), check);
      const emitted = generation.targets.reduce((total, target) => total + target.emittedSources.length, 0);
      const excluded = generation.targets.reduce((total, target) => total + target.sourceExclusions.length, 0);
      const unsupported = generation.targets.reduce((total, target) => total + target.unsupportedSources.length, 0);
      process.stdout.write(
        `${check ? 'Verified' : 'Generated'} ${inventory.summary.packages} inventoried packages; emitted ${String(emitted)} Rust modules, with ${String(excluded)} explicit exclusions and ${String(unsupported)} unsupported sources.\n`,
      );
    } else {
      process.stdout.write(
        `${check ? 'Verified' : 'Inventoried'} ${inventory.summary.packages} packages and ${inventory.summary.exports} public exports.\n`,
      );
    }
  }
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(`${message}\n`);
  process.exitCode = 1;
}
