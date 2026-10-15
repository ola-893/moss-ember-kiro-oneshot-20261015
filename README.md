# Moss & Ember

**Fictional demo café and bakery website.** Moss & Ember is not a real business. All
addresses, opening hours, prices, and names are invented.

A polished three-page static website built with [Fission](https://fission.rs) 0.14.1 and Rust,
deployed to GitHub Pages.

## Pages

| Route | Title |
|---|---|
| `/` | Home — hero, specials, intro, CTA |
| `/menu/` | Menu — coffee, pastries, light bites, lunch |
| `/visit/` | Visit — hours, address, directions, accessibility, about |
| `/content/about/` | About this demo |

## Tech

- **Language:** Rust (2021 edition)
- **UI framework:** [Fission](https://fission.rs) 0.14.1, `static-site` target
- **Deployment:** GitHub Pages via GitHub Actions

## Build locally

```sh
# Install the CLI (once)
cargo install cargo-fission --version 0.14.1 --locked

# Check routes
fission site routes --project-dir .

# Release check
fission site check --project-dir . --release

# Package
fission package --project-dir . --target static-site --format static --release

# Local preview
fission site serve --project-dir . --host 127.0.0.1 --port 8123
```

Package output lands in `target/fission/release/static-site/static/`. Do not commit it.

## Deployment

Pushes to `main` trigger the `fission-pages.yml` GitHub Actions workflow, which:

1. Installs `cargo-fission 0.14.1` on the runner
2. Checks and packages the static site
3. Verifies required output files
4. Uploads and deploys to GitHub Pages

Live URL: <https://ola-893.github.io/moss-ember-kiro-oneshot-20261015/>

## Licence

Apache-2.0 (same as the Fission framework). Demo content is entirely fictional.
