# cjames-web

My personal site — [cjameshawkins.com](https://cjameshawkins.com) — running entirely
in the browser as a Python WebAssembly app via [PyScript](https://pyscript.net)
(Pyodide) and the [PuePy](https://puepy.dev) framework. Blog posts are authored as
markdown and rendered **client-side with Python's `markdown` package**.

## Stack

- **PuePy** — reactive, Vue-inspired Python frontend framework
- **PyScript / Pyodide** — CPython compiled to WASM, running in the browser
- **Tailwind CSS** (CDN) for styling
- **Hatch** — environment management and local/CI workflows

## Project layout

```
index.html       PyScript scaffold (loads main.py + pyscript.toml)
main.py          builds the app, restores theme, mounts to #app
app/             blog data layer, components, and pages
posts/*.md       blog posts (front matter + markdown)
build.py         regenerates posts/index.json + pyscript.toml, assembles dist/
pyproject.toml   Hatch config (environments + scripts)
tests/           pytest suite for the data layer
```

## Local development

This project uses [Hatch](https://hatch.pypa.io).

```bash
hatch run build     # regenerate the manifest/config and assemble dist/
hatch run serve     # build, then serve dist/ at http://localhost:8000
hatch test          # run the data-layer test suite
hatch run clean     # remove dist/
```

Before committing, run the checks:

```bash
hatch check code    # lint (Ruff)
hatch check fmt     # format check — add --fix to apply formatting
hatch check types   # type check (Pyrefly)
```

Type checking treats `js`/`pyodide` (browser-injected) and the untyped, highly
dynamic `puepy` framework as `Any`, so the checked surface is our own logic in
`app/blog.py` and `build.py`. Lint/format use the project's Ruff config in
`pyproject.toml` (`[tool.ruff]`).

> PyScript must be served over HTTP (not `file://`). `hatch run serve` handles
> that. Note: deep-linking to `/blog/<slug>` only works in production — GitHub
> Pages serves `404.html` (a copy of `index.html`) as the SPA fallback, which a
> plain local HTTP server does not.

## Writing a post

Drop a markdown file in `posts/` with front matter, then rebuild:

```markdown
---
title: My Post
author: C. James
date: 2026-01-15T12:00:00Z
tags: [python, wasm]
---

Markdown body…
```

```bash
hatch run build
```

## Deployment

Pushing to `main` builds with Hatch and publishes `dist/` to GitHub Pages
(see `.github/workflows/continuous_deployment.yml`).
