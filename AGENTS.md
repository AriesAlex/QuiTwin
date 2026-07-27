# QuiTwin contributor guide

QuiTwin is a Windows x64 Rust application that acts as both a portable installer and Discord's installed `Update.exe` launch entrypoint. It installs and launches Equicord through an NTFS hardlink shadow runtime while leaving Discord's real application host updateable.

## Core invariants

- Keep Discord's real `app-*` host and live `resources/app.asar` stock. Do not move, replace, or persistently patch them.
- Preserve Discord's native update flow, direct post-update restart, Windows Settings uninstall, Stable/PTB/Canary discovery, and installs outside `C:`.
- Keep QuiTwin resident-free: no service, scheduled task, tray process, or permanent watcher.
- Preserve the one-binary model. The portable file installs the same binary as `Update.exe`; installed mode launches Discord, performs cleanup, and handles uninstall.
- Runtime generations under `.quitwin/runtime` are disposable and immutable after publication. Build into staging paths and publish atomically.
- A failed or interrupted operation must leave either the previous valid state or a stock Discord installation, never a half-written live launcher or host.
- Portable self-deletion happens only after the installed launcher has verified the source process has exited and the installed copy matches it.

## Before changing behavior

- Inspect the current path with `rg`/`rg --files` before adding a helper, abstraction, or fallback.
- Trace the real Windows and Discord lifecycle end to end: portable process, installed `Update.exe`, shadow generation, Discord child process, updater restart, and uninstall entry.
- Treat source code, actual filesystem state, registry values, process arguments, logs, and a reproducible launch/update as stronger evidence than installer names or assumptions about Squirrel.
- Do not mutate a developer's live Discord installation, force an update, close Discord, or test uninstall without explicit permission. Use temporary directories for isolated tests.
- Keep Windows API details in `src/platform.rs`, Discord discovery and version rules in `src/discord.rs`, runtime construction in `src/shadow.rs`, and orchestration in `src/install.rs`.

## Code style

- Prefer the cleanest final design over the smallest patch. Remove dead imports, obsolete branches, temporary wrappers, and traces of disproved hypotheses.
- Keep one source of truth. Compute state when possible instead of storing parallel flags or duplicating version/path rules.
- Do not add speculative compatibility branches, silent catches, or "just in case" fallbacks. Optional behavior must represent a real supported state.
- Add error context at the boundary where an operation can fail; keep the diagnostic log actionable without exposing secrets.
- Keep unsafe and raw Win32 calls narrowly scoped and explain non-obvious lifetime, handle, and atomicity requirements.
- Avoid one-line helper functions that do not clarify ownership or intent.

## Landing site

- `site/` is a Nuxt 3 static site built with pnpm, Vue, and SCSS. Do not introduce React, Tailwind, or a UI framework.
- Keep Vue SFCs ordered as `<template>`, `<script>`, then `<style>`.
- Keep component styles next to their markup in `<style scoped lang="scss">`. Put only shared tokens, resets, and genuinely global rules in `site/assets/styles/`; do not grow a detached page-level stylesheet.
- Use nested SCSS where it makes the component hierarchy clearer, without BEM-style class names or deeply coupled selectors.
- Keep this landing page to the shortest shape that explains and downloads the utility. Its default information architecture is one self-contained hero, not a marketing funnel.
- Keep the design restrained and product-specific. Do not add eyebrow labels above headings, AI-style gradients, ornamental blur, generic feature-card grids, nested cards, trust strips, proof ledgers, decorative stat badges, technical metrics, or numbered markers that do not communicate necessary sequence or state.
- Do not scaffold pages as a routine hero / three benefits / testimonials / CTA stack. Do not expand a small utility into a long SaaS-style sales page.
- Use whitespace and typography for grouping. Borders must communicate a real boundary or state; never use them as page scaffolding or wrap every section, control, and fact in a rectangle.
- Keep interactive shapes and spacing consistent. Prefer calm filled controls and avoid a page made from boxes inside boxes.
- Prefer flex for simple one-dimensional layout and use grid only when the content has a real two-dimensional relationship.
- Use the project's icon library for interface and brand icons instead of hand-drawn SVGs or text glyph substitutes.
- Keep the GitHub Pages base path configurable through `NUXT_APP_BASE_URL`; never hardcode asset paths that break `/QuiTwin/`.
- All user-facing copy and metadata must exist for EN, RU, SR, PL, TR, FR, AR, and ZH. Keep locale objects centralized in `site/shared/locales.ts`.
- Treat translations as native copy adaptations, not line-by-line translations. Preserve meaning and brevity while using natural phrasing for each locale.
- Browser-language detection must run before the first paint on the unprefixed static route. Never accept a visible English render followed by a client-side locale redirect.
- Every locale must remain prerendered and indexable with the correct `lang`, `dir`, canonical URL, hreflang alternates, Open Graph metadata, and JSON-LD.
- Keep RTL and both desktop and mobile layouts visually verified in the user's real browser. Fix layout from parent structure downward rather than compensating with negative offsets.
- Keep CSS manually understandable: clear variables, simple breakpoints, no compensating negative offsets, and modern functions only where they remove duplication.

## Verification

Run the checks appropriate to every touched area before committing:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo build --release --locked

cd site
pnpm install --frozen-lockfile
pnpm typecheck
$env:NUXT_APP_BASE_URL='/QuiTwin/'
pnpm generate
pnpm verify:output
```

For launcher changes, also test the real installed path when permission exists: ordinary launch, forced Discord host update, direct updater restart, next normal launch, portable self-deletion, and Windows Settings uninstall registration. Record proof in logs rather than adding test-only UI.

For site changes, inspect the generated `.output/public` artifact and verify the real render in at least one desktop and one mobile viewport. Reuse one browser session and do not leave preview processes running.

Before handoff, review `git diff` and `git status --short` as a reviewer. Confirm there are no unrelated files, unused code, broken translations, doubled GitHub Pages base paths, missing notices for embedded third-party assets, or intermediate artifacts.

## Releases

- `Cargo.toml` is the version source. Release tags must be exactly `v<version>`.
- Let GitHub Actions build the public binary from the tagged source. Do not publish a redundant `.sha256` sidecar when GitHub exposes the release asset digest.
- Do not call a release complete until CI, Pages deployment, the GitHub release, the direct latest-download URL, and GitHub's published asset digest have all been verified from the public endpoints.
