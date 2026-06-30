import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const rootDir = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const frontendDir = resolve(rootDir, 'frontend');
const backendDir = resolve(rootDir, 'backend');
const npmCommand = process.platform === 'win32' ? 'npm.cmd' : 'npm';

function run(command, args, cwd) {
  const options = { cwd, stdio: 'inherit' };
  const result =
    process.platform === 'win32'
      ? spawnSync([command, ...args].join(' '), { ...options, shell: true })
      : spawnSync(command, args, options);

  if (result.error) {
    console.error(`Failed to run ${command}: ${result.error.message}`);
    process.exit(1);
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

if (!existsSync(resolve(frontendDir, 'node_modules'))) {
  run(npmCommand, ['ci'], frontendDir);
}

run(npmCommand, ['run', 'build'], frontendDir);
run('cargo', ['build', '--release'], backendDir);
