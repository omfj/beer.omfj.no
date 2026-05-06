# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Track drinks at events with friends. Create an event, log beers, wines, shots, and cocktails, and compete on a real-time leaderboard. Events can be password-protected.

## Development Commands

```bash
pnpm dev                    # Start dev server (Vite)
pnpm build                  # Build for production
pnpm preview                # Build and preview with Wrangler
pnpm test:unit              # Run tests in watch mode
pnpm test                   # Run tests once
pnpm lint                   # Check code with Prettier and ESLint
pnpm lint:fix               # Auto-fix linting issues
pnpm check                  # SvelteKit sync and type check
pnpm check:watch            # Type check in watch mode

# Database commands (Drizzle ORM)
pnpm db:push                # Push schema changes to database
pnpm db:studio              # Open Drizzle Studio GUI
pnpm db:generate            # Generate migrations
pnpm db:migrate:local       # Apply migrations locally
pnpm db:migrate:remote      # Apply migrations to remote

pnpm deploy                 # Build and deploy to Cloudflare Pages
pnpm typegen                # Generate Wrangler types
```

## Architecture

### Database Schema (Drizzle ORM with Cloudflare D1)

Defined in `src/lib/db/schema.ts`:

- **users**: Username, type, gender, weight, terms acceptance, creation timestamp
- **userPasswords**: Password hashes (separate table, cascade delete)
- **sessions**: User sessions (30-day expiry, auto-renewed at 15 days). Cookie name: `auth-session`
- **events**: Name, color, optional password, creator, timestamp
- **eventAccess**: Tracks which users have unlocked password-protected events (eventId, userId, grantedAt)
- **attendees**: One record per drink logged — links user + event + drink type + size + optional image + optional ABV override
- **drinkTypes**: Drink categories (beer, wine, cocktail, shot) with ABV percentage and score multiplier
- **drinkSizes**: Drink volumes in mL (e.g., 0.33L, 0.5L, glass of wine)
- **drinkTypeSizes**: Junction table mapping valid type-size combinations

### Authentication System

Custom session-based auth in `src/lib/auth.ts`:

- Session tokens are 18-byte random values, base64url encoded
- Session IDs are SHA-256 hashes of tokens (stored in DB)
- Hook in `src/hooks.server.ts` validates sessions on every request
- DB instance, R2 bucket, and session service injected into `event.locals`

### Event Access Control

Password-protected events use bcryptjs. Logic in `src/lib/event-access.ts`:

- Users must unlock a protected event before viewing it (`/arrangement/[id]/unlock`)
- Access is recorded in the `eventAccess` table
- The global leaderboard (`/toppliste`) only shows public (non-password-protected) events

### Scoring System

Located in `src/lib/scoring.ts`:

- Formula: `((Volume (L) × ABV (decimal) × 0.789 × 1000) / 10) × multiplier`
- `multiplier` comes from the drinkType (defaults to 1.0)
- Returns 0.5 points if volume or ABV is missing/invalid (backwards compatibility)
- 0.789 = ethanol density in g/mL
- Result rounded to 1 decimal place

### SvelteKit Route Structure

Routes in `src/routes`:

- `(app)/(auth)/*`: Protected routes (require login)
  - `/`: Home — user's events
  - `/arrangement/[id]`: Event detail with drink registrations
  - `/arrangement/[id]/registrer`: Log a drink
  - `/arrangement/[id]/qr`: QR code for sharing
  - `/arrangement/[id]/unlock`: Unlock a password-protected event
  - `/arrangementer/ny`: Create new event
  - `/toppliste`: Global leaderboard (public events only)
  - `/endringer`: Changelog
  - `/profil`: User profile
  - `/logg-ut`: Logout
- `(app)/(public)/*`: Public routes
  - `/logg-inn`: Login
  - `/registrer`: Register account
  - `/vilkar`: Terms and conditions
- `api/image/[id]`: Serve images from R2

Route groups `(app)`, `(auth)`, `(public)` are layout groupings and don't affect URLs.

### Platform Environment Access

Cloudflare bindings (see `src/app.d.ts`):

- `env.DB`: Cloudflare D1 database
- `env.BUCKET`: R2 bucket for image storage
- Access via `event.platform.env` in endpoints, or `event.locals` after hooks

### Migration Strategy

Drizzle Kit manages migrations in `migrations/`:

- Generate: `pnpm db:generate`
- Apply locally: `pnpm db:migrate:local`
- Apply remote: `pnpm db:migrate:remote`
- Some migrations include data seeding (e.g., `0010_seed-drink-types-sizes.sql`)

## Key Technical Decisions

- **Cloudflare Stack**: D1 (SQLite), R2 (object storage), Pages (hosting), Durable Objects (WebSockets)
- **No Traditional Backend**: SvelteKit server-side + Cloudflare bindings
- **Custom Auth**: Hand-rolled session management, no third-party auth library
- **Type Safety**: TypeScript throughout with Drizzle for type-safe queries
