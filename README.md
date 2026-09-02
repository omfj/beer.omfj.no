# beer.omfj.no

Track drinks at events with friends. Create an event, log beers, wines, shots, and cocktails, and compete on a real-time leaderboard.

This repository is a pnpm monorepo with two members:

- `web`: the existing SvelteKit frontend and Cloudflare application
- `api`: an Axum API backed by SQLx and SQLite

## Development

```bash
pnpm install
pnpm dev
```

The web app runs on Vite's default address and the API listens on port `3000`. The API exposes `GET /health`. Copy `api/.env.example` into your preferred environment setup or export those variables before starting it.

You can also work on one member at a time:

```bash
pnpm --filter @beer/web dev
pnpm --filter @beer/api dev
```

Build or check the complete workspace with `pnpm build` or `pnpm check`.
