#!/usr/bin/env node
/**
 * Post-tool-use hook: Automatic cargo fmt on Rust file modifications
 *
 * This hook runs after Write or Edit tool calls and formats Rust files using cargo fmt.
 * It ensures all Rust code stays formatted according to project standards.
 *
 * Input (stdin): JSON with tool input containing file_path
 * Output: JSON with systemMessage on success/failure
 */

import { spawn } from 'child_process';
import { join } from 'path';

async function main() {
  // Read stdin
  const chunks = [];
  for await (const chunk of process.stdin) {
    chunks.push(chunk);
  }
  const input = Buffer.concat(chunks).toString('utf8');

  let toolData;
  try {
    toolData = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  // Extract file path from tool call
  const filePath = toolData?.input?.file_path;

  if (!filePath) {
    process.exit(0);
  }

  // Check if it's a Rust file
  if (!filePath.endsWith('.rs')) {
    process.exit(0);
  }

  // Build cargo fmt command
  const projectDir = process.env.CLAUDE_PROJECT_DIR;
  const args = ['fmt', '--'];

  if (projectDir) {
    args.splice(1, 0, '--manifest-path', join(projectDir, 'Cargo.toml'));
  }

  args.push(filePath);

  // Run cargo fmt
  const result = await new Promise((resolve) => {
    const proc = spawn('cargo', args, {
      stdio: ['ignore', 'pipe', 'pipe'],
      shell: process.platform === 'win32',
    });

    let stdout = '';
    let stderr = '';

    proc.stdout?.on('data', (data) => { stdout += data; });
    proc.stderr?.on('data', (data) => { stderr += data; });

    proc.on('error', (err) => {
      resolve({ code: 1, output: err.message });
    });

    proc.on('close', (code) => {
      resolve({ code: code || 0, output: stderr || stdout });
    });
  });

  if (result.code === 0) {
    console.log(JSON.stringify({
      systemMessage: `✓ Formatted ${filePath}`,
    }));
  } else {
    console.log(JSON.stringify({
      systemMessage: `⚠ cargo fmt issues with ${filePath}: ${result.output}`,
    }));
  }

  process.exit(0);
}

main();
