# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a T3 Stack application (Next.js 15 + tRPC + Drizzle ORM + NextAuth) for a healthy diet tracking application.

## Development Commands

```bash
# Development
npm run dev              # Start dev server with Turbo
npm run build           # Production build
npm run start           # Start production server
npm run preview         # Build and start production server

# Code Quality
npm run lint            # Run ESLint
npm run lint:fix        # Fix ESLint issues
npm run typecheck       # Run TypeScript type checking
npm run check           # Run both ESLint and typecheck
npm run format:check    # Check code formatting
npm run format:write    # Format code with Prettier

# Database (Drizzle)
npm run db:generate     # Generate migrations from schema
npm run db:migrate      # Apply migrations
npm run db:push         # Push schema changes directly to DB
npm run db:studio       # Open Drizzle Studio
```

## Architecture

### tRPC Setup

- **Server-side**: tRPC routers are defined in `src/server/api/routers/`
- **Client-side**: Use `api` from `@/trpc/react` for React components, or `api` from `@/trpc/server` for server components
- **Root router**: All routers must be registered in `src/server/api/root.ts` (appRouter)
- **Procedures**: Use `publicProcedure` or `protectedProcedure` from `@/server/api/trpc`
- **Context**: Available in procedures - includes `db`, `session`, and `headers`

### Database (Drizzle + PostgreSQL)

- **Schema**: Defined in `src/server/db/schema.ts`
- **Multi-project schema**: Tables are prefixed with `healthy-diet_` via `pgTableCreator`
- **DB instance**: Import `db` from `@/server/db`
- **After schema changes**: Run `npm run db:generate` then `npm run db:push`

### Authentication (NextAuth v5)

- **Config**: Located in `src/server/auth/config.ts`
- **Providers**: Currently uses Discord OAuth (configured via AUTH_DISCORD_ID/SECRET)
- **Session**: Access via `auth()` function from `@/server/auth`
- **Protected routes**: Use `protectedProcedure` in tRPC or check session manually

### Environment Variables

- **Schema**: Defined in `src/env.js` using @t3-oss/env-nextjs
- **Required vars**: DATABASE_URL, AUTH_DISCORD_ID, AUTH_DISCORD_SECRET, AUTH_SECRET
- **Adding new vars**: Update both `src/env.js` schema and `.env.example`

### File Structure

- `src/app/` - Next.js App Router pages and layouts
- `src/server/api/routers/` - tRPC route handlers
- `src/server/api/trpc.ts` - tRPC initialization and procedures
- `src/server/auth/` - NextAuth configuration
- `src/server/db/` - Database schema and client
- `src/trpc/` - tRPC client setup (React and server)

### Key Patterns

- **Transformer**: SuperJSON is configured for tRPC to handle Date, Map, Set, etc.
- **Error handling**: ZodErrors are automatically formatted and sent to frontend
- **Dev timing**: Artificial delay (100-500ms) added in dev to catch waterfall issues
