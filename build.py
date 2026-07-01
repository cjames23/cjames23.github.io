#!/usr/bin/env python3
"""Build step for the PyScript site (stdlib only).

- Discovers ``posts/*.md`` and writes ``posts/index.json`` (the slug manifest the
  browser reads to know which posts exist).
- Regenerates ``pyscript.toml`` so every app module and post is bundled into the
  Pyodide virtual filesystem.
- Assembles a clean ``dist/`` for GitHub Pages deployment (with 404.html SPA
  fallback, .nojekyll, and CNAME).
"""

import glob
import json
import os
import shutil

ROOT = os.path.dirname(os.path.abspath(__file__))
POSTS = os.path.join(ROOT, "posts")
DIST = os.path.join(ROOT, "dist")

PACKAGES = ["puepy", "markdown"]
APP_MODULES = ["app/__init__.py", "app/blog.py", "app/components.py", "app/pages.py"]
STATIC_FILES = ["index.html", "pyscript.toml", "main.py", "style.css", "CNAME"]


def discover_slugs():
    return [os.path.splitext(os.path.basename(p))[0] for p in sorted(glob.glob(os.path.join(POSTS, "*.md")))]


def write_index(slugs):
    with open(os.path.join(POSTS, "index.json"), "w") as fh:
        json.dump(slugs, fh, indent=2)
        fh.write("\n")


def write_pyscript_toml(slugs):
    # The KEY is the URL PyScript fetches; the VALUE is the path in the Pyodide
    # VFS. PyScript resolves relative fetch URLs against `document.URL`, NOT
    # `document.baseURI`, so a deep-link route like `/blog/<slug>` would try to
    # fetch `/blog/app/__init__.py`. Root-anchored URLs sidestep that.
    packages = ", ".join(repr(p) for p in PACKAGES)
    lines = [f"packages = [{packages}]", "", "[files]"]
    lines.extend(f'"/{module}" = "{module}"' for module in APP_MODULES)
    lines.append('"/posts/index.json" = "posts/index.json"')
    lines.extend(f'"/posts/{slug}.md" = "posts/{slug}.md"' for slug in slugs)
    with open(os.path.join(ROOT, "pyscript.toml"), "w") as fh:
        fh.write("\n".join(lines) + "\n")


def assemble_dist(slugs):
    if os.path.exists(DIST):
        shutil.rmtree(DIST)
    os.makedirs(DIST)

    for name in STATIC_FILES:
        src = os.path.join(ROOT, name)
        if os.path.exists(src):
            shutil.copy(src, os.path.join(DIST, name))

    shutil.copytree(
        os.path.join(ROOT, "app"),
        os.path.join(DIST, "app"),
        ignore=shutil.ignore_patterns("__pycache__", "*.pyc"),
    )

    os.makedirs(os.path.join(DIST, "posts"))
    shutil.copy(os.path.join(POSTS, "index.json"), os.path.join(DIST, "posts", "index.json"))
    for slug in slugs:
        shutil.copy(os.path.join(POSTS, f"{slug}.md"), os.path.join(DIST, "posts", f"{slug}.md"))

    # GitHub Pages SPA fallback + disable Jekyll.
    shutil.copy(os.path.join(ROOT, "index.html"), os.path.join(DIST, "404.html"))
    open(os.path.join(DIST, ".nojekyll"), "w").close()


def main():
    slugs = discover_slugs()
    write_index(slugs)
    write_pyscript_toml(slugs)
    assemble_dist(slugs)
    print(f"Built {len(slugs)} post(s): {', '.join(slugs) or '(none)'}")
    print("Regenerated posts/index.json and pyscript.toml; assembled dist/.")


if __name__ == "__main__":
    main()
