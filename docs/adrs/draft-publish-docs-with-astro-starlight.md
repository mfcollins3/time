---
status: "proposed"
date: 2026-11-21
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Publish product documentation with Astro Starlight on GitHub Pages

## Context and Problem Statement

Naked Time's only user-facing documentation is the repository `README.md`. Users
need product documentation that explains how to install and use Naked Time. How
do we author, build, and host that documentation with minimal cost and
maintenance?

## Decision Drivers

* Documentation should be written in Markdown and live in the same repository
  as the product, so it can change in the same pull requests as the code.
* Hosting should be free and need no infrastructure to maintain.
* The site should have navigation, search, and a good reading experience out of
  the box.
* Publishing should be automatic when documentation changes on `main`.

## Considered Options

* Astro with the Starlight theme, hosted on GitHub Pages
* mdBook, hosted on GitHub Pages
* A hosted documentation service

## Decision Outcome

Chosen option: "Astro with the Starlight theme, hosted on GitHub Pages",
because it gives a polished documentation site with search, sidebar navigation,
and dark mode with little configuration, keeps content as Markdown in the
repository, and needs no hosting beyond GitHub Pages.

The details of the decision are:

* The site lives in the `website/` directory at the repository root and is
  served from `https://mfcollins3.github.io/time/` (`site` and `base: '/time'`
  in `astro.config.mjs`).
* Dependencies are managed with pnpm; `pnpm-lock.yaml` is committed and
  `packageManager` is pinned in `package.json`.
* The `docs` GitHub Actions workflow builds the site with `withastro/action`
  on pull requests and pushes to `main` that change `website/`, and deploys it
  with `actions/deploy-pages` only on pushes to `main` (and manual dispatch).
* The repository's Pages source is set to "GitHub Actions".

### Consequences

* Good, because documentation changes are reviewed and versioned with the code.
* Good, because pull requests verify that the site builds before merge.
* Good, because hosting is free.
* Bad, because the repository now has a Node.js toolchain alongside Rust. It is
  confined to `website/` and is not needed to build the product.
* Bad, because Astro requires a recent Node.js release (22.23.4 or later), so
  contributors may need to upgrade Node.js to work on the site.
* Bad, because the site is served under the `/time` path, so internal links
  must be relative or base-aware, and a custom domain would require changing
  `base`.

### Confirmation

The `docs` workflow's build job must pass on every pull request that changes
`website/`, and a successful run on `main` publishes the site.

## Pros and Cons of the Options

### Astro with the Starlight theme, hosted on GitHub Pages

* Good, because search, navigation, and theming are built in.
* Good, because it can be extended with components and plugins as needs grow.
* Bad, because it adds a Node.js dependency tree to maintain.

### mdBook, hosted on GitHub Pages

* Good, because it is a single Rust binary and fits the existing toolchain.
* Neutral, because it also publishes to GitHub Pages.
* Bad, because it offers less flexibility for landing pages and customization.

### A hosted documentation service

* Good, because there is nothing to build or deploy.
* Bad, because it adds cost and an external account, and ties content to a
  vendor.
