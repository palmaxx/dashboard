# Qwen Dashboard (Next.js + Backend)
A working dashboard implementation based on `../qwenapp.txt`, rebuilt as a real React + Next.js app with a backend API for live hardware metrics.
## Current basic features
- Live clock/date header.
- Real hardware stats from backend API:
  - CPU usage, model, cores, frequency, temperature (when available)
  - RAM usage
  - GPU model, usage/temperature/VRAM (when available)
  - Network upload/download rates and interface state
  - Storage usage per drive
- Favorite websites widget (localStorage persisted).
- Todo list widget (localStorage persisted).
## Not included yet
- Stocks or any third-party API data.
- Downloader, weather API, or other advanced integrations.
## Tech stack
- Next.js App Router
- React
- Tailwind CSS v4
- `systeminformation` for backend hardware metrics
## Run locally
1. Install dependencies:
   `npm install`
2. Start development server:
   `npm run dev`
3. Open:
   [http://localhost:3000](http://localhost:3000)
## Production build check
- `npm run lint`
- `npm run build`
## API endpoint
- `GET /api/hardware`
- Response shape:
  - `cpu`, `memory`, `gpu`, `network`, `storage`, `timestamp`
  - Values may be `null` when the OS/hardware does not expose a sensor (for example some CPU/GPU temperatures).
## Project structure
- Frontend dashboard page: `src/app/page.tsx`
- Backend hardware API: `src/app/api/hardware/route.ts`
- Global styles: `src/app/globals.css`
