# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri desktop application for managing tabletop RPG initiative and campaigns. It's built with:
- **Frontend**: SvelteKit 2 with TypeScript, Tailwind CSS (v4), and DaisyUI
- **Backend**: Rust with Tauri 2.4.1, SQLite database with r2d2 connection pooling
- **Package Manager**: pnpm

**Important**: 
- Always use the latest version of Svelte and follow current Svelte 5+ patterns including runes ($state, $derived, $effect) and modern component syntax
- Always use Tauri v2 APIs and patterns - import from `@tauri-apps/api/core` for invoke, `@tauri-apps/api/path` for paths, etc.
- Focus on creating simple, reusable components - break down complex components into smaller, focused pieces that can be easily composed and maintained

The app manages campaigns, encounters, and combattens (characters/NPCs) with a cyberpunk-themed UI.

## Development Commands

### Frontend Development
```bash
pnpm dev                 # Start Vite dev server (http://localhost:1420)
pnpm build              # Build frontend for production
pnpm preview            # Preview production build
pnpm check              # Run Svelte type checking
```

### Tauri Development
```bash
pnpm tauri dev          # Start Tauri development mode
pnpm tauri build        # Build Tauri app for production
```

### Styling
```bash
pnpm tailwind:watch     # Watch mode for Tailwind CSS development
pnpm tailwind:build     # Build Tailwind CSS for production
```

## Architecture

### Database Schema
The SQLite database has versioned migrations in `src-tauri/src/database.rs`:
- **campaigns**: id, name, version
- **encounters**: id, name, campaign_id
- **combattens**: id, name, type, physical, stun, campaign_id
- **encounter_combattens**: id, encounter_id, combatten_id, damage, initiative

### Tauri Commands
Backend commands are organized in `src-tauri/src/actions/`:
- **campaign.rs**: add_campaign, get_all_campaigns, get_campaign, remove_campaign
- **combatten.rs**: add_combatten, get_all_combattens, get_all_campaign_combattens, get_combatten, edit_combatten, remove_combatten
- **encounter.rs**: add_encounter, get_all_encounters, get_encounter, remove_encounter, add_combatten_to_encounter

### Frontend Structure
- **Routes**: SvelteKit file-based routing with dynamic routes for campaigns and encounters
- **Components**: Reusable components in `src/lib/` including forms, list items, and layout components
- **Types**: TypeScript interfaces auto-generated from Rust structs using ts-rs
- **Styling**: Cyberpunk theme with CSS custom properties for neon colors

### Key Configuration
- **Tauri Config**: Mobile-style app window (480x600px), SQLite in app data directory
- **SvelteKit**: Static adapter for Tauri, SSR disabled, prerendering enabled
- **Database**: WAL journal mode, connection pooling with r2d2

### Type Safety
TypeScript types are automatically generated from Rust structs using the ts-rs crate. When modifying Rust structs, regenerate TypeScript types by rebuilding the Rust project.