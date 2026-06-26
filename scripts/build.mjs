import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const rootDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const frontendDir = resolve(rootDir, 'frontend');
const backendDir = resolve(rootDir, 'backend');

function run(command, args, cwd) {
  const result = spawnSync(command, args, {
    cwd,
    stdio: 'inherit',
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const npmCommand = process.platform === 'win32' ? 'npm.cmd' : 'npm';

if (!existsSync(resolve(frontendDir, 'node_modules'))) {
  run(npmCommand, ['ci'], frontendDir);
}

run(npmCommand, ['run', 'build'], frontendDir);
run('cargo', ['build', '--manifest-path', 'Cargo.toml', '--release'], backendDir);