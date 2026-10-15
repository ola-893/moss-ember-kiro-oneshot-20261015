# Moss & Ember

A fictional neighbourhood café and bakery website, built as a demo of the
[Fission](https://fission.rs) static-site framework.

> **This is a demo site.** Moss & Ember is not a real business. All addresses,
> opening hours, prices and names are fictional.

## Pages

- **Home** — hero, bakery story, daily specials, CTA
- **Menu** — coffee, pastries, light bites and lunch with GBP prices
- **Visit** — opening hours, address, accessibility, café story

## Live site

<https://ola-893.github.io/moss-ember-kiro-oneshot-20261015/>

## Build locally

Requires Rust (stable) and the Fission CLI 0.14.1.

```sh
cargo install cargo-fission --version 0.14.1 --locked

fission site check --project-dir . --release
fission package --project-dir . --target static-site --format static --release
# Output: target/fission/release/static-site/static/
```

## Deploy

Push to `main`. The `.github/workflows/fission-pages.yml` workflow installs the
CLI, checks, packages and deploys to GitHub Pages automatically.

## Framework

Built with [Fission](https://fission.rs) — a Rust UI framework for desktop,
mobile and static web.
