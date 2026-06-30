const path = require('node:path');

const rootDir = __dirname;
const isWin = process.platform === 'win32';

module.exports = {
  apps: [
    {
      name: 'Dashboard Backend',
      cwd: rootDir,
      script: path.join(rootDir, 'backend', 'target', 'release', isWin ? 'backend.exe' : 'backend'),
      exec_mode: 'fork',
      instances: 1,
      autorestart: true,
      watch: false,
      max_restarts: 10,
      min_uptime: '5s',
      restart_delay: 2000,
      time: true,
      merge_logs: true,
      out_file: path.join(rootDir, 'logs', 'dashboard-backend.out.log'),
      error_file: path.join(rootDir, 'logs', 'dashboard-backend.err.log'),
      env: {
        RUST_BACKTRACE: '1',
        NO_COLOR: '1',
      },
    },
  ],
};
