# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Core Development

- `pnpm dev` - Start development server
- `pnpm dev --host` - Start development server accessible from other devices (mobile testing)
- `pnpm build` - Build for production
- `pnpm preview` - Build and preview with Wrangler
- `pnpm deploy` - Build and deploy to Cloudflare Workers

### Code Quality

- `pnpm lint` - Run Prettier and ESLint checks
- `pnpm format` - Format code with Prettier
- `pnpm check` - Run Svelte type checking
- `pnpm check:watch` - Run Svelte type checking in watch mode

### Testing

- `pnpm test:unit` - Run Vitest in watch mode
- `pnpm test` - Run tests once

### Database

- `pnpm db:push` - Push schema changes to database
- `pnpm db:studio` - Open Drizzle Studio
- `pnpm db:generate` - Generate migrations
- `pnpm db:migrate:local` - Apply migrations locally
- `pnpm db:migrate:remote` - Apply migrations to remote

### Cloudflare Types

- `pnpm typegen` - Generate Wrangler types

## Architecture Overview

This is a SvelteKit application deployed on Cloudflare Workers with:

### Technology Stack

- **Framework**: SvelteKit with TypeScript
- **Database**: SQLite (Cloudflare D1) with Drizzle ORM
- **Storage**: Cloudflare R2 for image uploads
- **Styling**: TailwindCSS v4
- **Authentication**: Custom session-based auth with secure tokens
- **Package Manager**: pnpm

### Database Schema

Core entities in `src/lib/db/schema.ts`:

- `users` - User accounts with username and terms agreement
- `userPasswords` - Separate table for password hashes
- `sessions` - Session management with 30-day expiry
- `events` - Beer counting events with colors
- `attendees` - Event participation records with optional images

### Authentication System

Located in `src/lib/auth.ts`:

- Session-based authentication using SHA-256 hashed tokens
- 30-day session expiry with automatic renewal
- Secure cookie handling via SessionService class
- User context available via `locals.user` in layouts

### File Structure

- `src/routes/` - SvelteKit pages and API endpoints
- `src/lib/db/` - Database schema and connection
- `src/lib/auth.ts` - Authentication logic
- `src/lib/components/` - Reusable Svelte components
- `src/lib/context/` - Svelte context providers
- `migrations/` - Drizzle database migrations

### Cloudflare Integration

Configured in `wrangler.jsonc`:

- D1 database binding as `DB`
- R2 bucket binding as `BUCKET`
- Custom domain: beer.omfj.no
- Assets served from SvelteKit build output

### Key Patterns

- Server-side data loading via `+page.server.ts` files
- Form actions for mutations in server files
- Image uploads stored in R2 with generated IDs
- Color-coded events for visual organization
