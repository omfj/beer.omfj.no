# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Beer Counter is a social drinking tracking application built as a Turborepo monorepo with two main applications:
- **apps/web**: SvelteKit web application with Cloudflare Pages adapter
- **apps/ws**: Cloudflare Durable Objects WebSocket server for real-time updates

The application allows users to create events, register drinks consumed with various types and sizes, and view real-time leaderboards. Users can track different drink types (beer, wine, cocktails, shots) with configurable sizes and ABV percentages.

## Development Commands

### Root Level (using Turbo)
```bash
pnpm dev          # Start all apps in development mode
pnpm build        # Build all apps
pnpm lint         # Lint all apps
pnpm format       # Format all apps
pnpm test         # Run tests in all apps
pnpm typegen      # Generate Cloudflare Worker types
```

### Web App (apps/web)
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

### WebSocket Server (apps/ws)
```bash
pnpm dev                    # Start Wrangler dev server
pnpm deploy                 # Deploy to Cloudflare Workers
pnpm typegen                # Generate Cloudflare Worker types
```

## Architecture

### Database Schema (Drizzle ORM with Cloudflare D1)

The database uses Drizzle ORM with SQLite (Cloudflare D1) and is defined in `apps/web/src/lib/db/schema.ts`:

- **users**: User accounts with username, type, terms acceptance, and creation timestamp
- **userPasswords**: Password hashes (separate table, cascade delete)
- **sessions**: User sessions with expiration (30-day expiry, auto-renewed at 15 days)
- **events**: Events/gatherings where drinks are tracked (name, color, creator, timestamp)
- **attendees**: Drink registrations (should conceptually be "registrations" - tracks each drink logged by a user at an event, not just attendance). Links user + event + drink type + size + optional image
- **drinkTypes**: Drink categories (beer, wine, cocktail, shot) with ABV percentage
- **drinkSizes**: Drink volumes in mL (e.g., 0.33L, 0.5L, glass of wine)
- **drinkTypeSizes**: Junction table mapping valid type-size combinations

Key relationships:
- Each attendee record represents one drink registration with type, size, and timestamp
- Events have many attendees (drink registrations)
- Users have many sessions and many attendees (drink registrations)
- Drink types and sizes are related via the drinkTypeSizes junction table

### Authentication System

Custom session-based authentication in `apps/web/src/lib/auth.ts`:
- SessionService class handles token generation, validation, and cookie management
- Session tokens are 18-byte random values, base64url encoded
- Session IDs are SHA-256 hashes of tokens (stored in database)
- Sessions expire after 30 days, auto-renewed if within last 15 days
- Hook in `apps/web/src/hooks.server.ts` validates sessions on every request
- Database instance, R2 bucket, and session service injected into event.locals
- User and session data attached to event.locals for route access

### WebSocket Real-Time Updates

The WebSocket system uses Cloudflare Durable Objects for real-time event updates:

**apps/ws** (Durable Objects WebSocket Server):
- `src/index.ts`: Hono app with two endpoints:
  - `GET /event/:id`: WebSocket connection upgrade for clients to subscribe to event updates
  - `POST /event/:id`: Broadcast endpoint (API-key protected) to notify all subscribers
- `src/room.ts`: WebSocketRoom Durable Object class maintains connections per event
- Each event has its own Durable Object instance (identified by event ID)
- Validates event exists before allowing connections
- Broadcasts simple "UPDATE" message to all connected clients

**apps/web** (Client Integration):
- `src/lib/ws.ts`: Client WebSocket manager
- When a drink is registered, the web app calls the WebSocket server's POST endpoint
- All connected clients receive "UPDATE" message and refresh their data
- Used for real-time leaderboard updates on event dashboard pages

### Scoring System

Located in `apps/web/src/lib/scoring.ts`:
- Points calculated by formula: `(Volume (L) × ABV (decimal) × 0.789 × 1000) / 10`
- Returns 0.5 points if volume or ABV is missing/invalid (for backwards compatibility)
- The 0.789 factor represents ethanol density in g/mL
- Result is rounded to 1 decimal place

### SvelteKit Route Structure

Routes in `apps/web/src/routes`:
- `(app)/(auth)/*`: Protected routes requiring authentication
  - `/`: Home/dashboard showing user's events
  - `/arrangement/[id]`: Event detail page with registrations
  - `/arrangement/[id]/registrer`: Register a new drink for the event
  - `/arrangement/[id]/qr`: QR code for event sharing
  - `/arrangementer/ny`: Create new event
  - `/profil`: User profile page
  - `/logg-ut`: Logout
- `(app)/(public)/*`: Public routes
  - `/logg-inn`: Login page
  - `/registrer`: User registration
  - `/vilkar`: Terms and conditions
- `arrangement/[id]/dashboard`: Real-time event leaderboard (WebSocket connected)
- `api/image/[id]`: Image serving endpoint (from R2 bucket)

Route groups `(app)`, `(auth)`, `(public)` are layout groupings (parentheses mean they don't affect URL structure).

### Platform Environment Access

The app runs on Cloudflare Pages/Workers with these platform bindings (see `apps/web/src/app.d.ts`):
- `env.DB`: Cloudflare D1 database
- `env.BUCKET`: R2 bucket for image storage
- `env.API_KEY`: API key for WebSocket server authentication
- Access via `event.platform.env` in endpoints
- Access via `event.locals` after hooks process them

### Migration Strategy

Drizzle Kit manages database migrations in `apps/web/migrations/`:
- Generate: `pnpm db:generate` creates migration files from schema changes
- Apply locally: `pnpm db:migrate:local` (uses Wrangler local mode)
- Apply remote: `pnpm db:migrate:remote` (applies to production D1)
- Migration files include both schema DDL and data seeding (see `0010_seed-drink-types-sizes.sql`)

## Key Technical Decisions

- **Cloudflare Stack**: D1 (SQLite), R2 (object storage), Pages (hosting), Durable Objects (WebSockets)
- **Monorepo**: Turborepo for managing web app and WebSocket server together
- **No Traditional Backend**: Leverages SvelteKit's server-side capabilities and Cloudflare bindings
- **Custom Auth**: Hand-rolled session management (no third-party auth library)
- **Real-time**: Durable Objects provide stateful WebSocket rooms per event
- **Type Safety**: TypeScript throughout, with Drizzle for type-safe database queries
