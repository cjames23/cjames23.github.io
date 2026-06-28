"""Blog data layer.

Posts are authored as markdown files in ``posts/`` with a small front-matter
header (the same format used by the previous Rust version). They are bundled
into the Pyodide virtual filesystem via ``pyscript.toml`` and rendered to HTML
*in the browser* with the pure-Python ``markdown`` package -- which is the whole
point of the Python-WASM rewrite.
"""

import json
import re
from datetime import datetime, timezone

import markdown

POSTS_DIR = "posts"
MARKDOWN_EXTENSIONS = ["extra", "sane_lists", "smarty"]


class BlogPost:
    def __init__(self, slug, title, body, author, created_at, updated_at, tags):
        self.slug = slug
        self.id = slug  # kept for parity with the old model
        self.title = title
        self.body = body
        self.author = author
        self.created_at = created_at
        self.updated_at = updated_at
        self.tags = tags

    @property
    def formatted_date(self):
        return self.created_at.strftime("%B %d, %Y")

    @property
    def reading_time(self):
        return max(1, len(self.plain_text().split()) // 220)

    def html(self):
        """Render the markdown body to an HTML string (client-side)."""
        return markdown.markdown(self.body, extensions=MARKDOWN_EXTENSIONS, output_format="html")

    def plain_text(self):
        """Markdown stripped down to rough plain text, for excerpts/reading time."""
        text = re.sub(r"`{1,3}", "", self.body)
        text = re.sub(r"[#>*_\[\]()!]", " ", text)
        return re.sub(r"\s+", " ", text).strip()

    def excerpt(self, max_chars=280):
        text = self.plain_text()
        if len(text) <= max_chars:
            return text
        return text[:max_chars].rstrip() + "…"


class BlogDb:
    """Loads and indexes the bundled markdown posts. Sorted newest-first."""

    def __init__(self):
        self.posts = []
        self._load()

    def _load(self):
        try:
            with open(f"{POSTS_DIR}/index.json") as fh:
                slugs = json.load(fh)
        except OSError:
            slugs = []

        posts = []
        for slug in slugs:
            try:
                with open(f"{POSTS_DIR}/{slug}.md") as fh:
                    raw = fh.read()
            except OSError:
                continue
            post = _parse_post(slug, raw)
            if post is not None:
                posts.append(post)

        posts.sort(key=lambda p: p.created_at, reverse=True)
        self.posts = posts

    def all(self):
        return self.posts

    def by_slug(self, slug):
        for post in self.posts:
            if post.slug == slug:
                return post
        return None

    def adjacent(self, slug):
        """Return ``(newer, older)`` posts relative to ``slug``."""
        index = next((i for i, p in enumerate(self.posts) if p.slug == slug), None)
        if index is None:
            return None, None
        newer = self.posts[index - 1] if index > 0 else None
        older = self.posts[index + 1] if index + 1 < len(self.posts) else None
        return newer, older


def _parse_post(slug, raw):
    raw = raw.lstrip("﻿")
    lines = raw.splitlines()
    if not lines or lines[0].strip() != "---":
        return None

    meta = {"title": slug, "author": "C. James", "date": "", "updated": None, "tags": []}
    i = 1
    while i < len(lines) and lines[i].strip() != "---":
        line = lines[i]
        i += 1
        if ":" not in line:
            continue
        key, _, value = line.partition(":")
        key = key.strip()
        value = value.strip()
        if key == "title":
            meta["title"] = _unquote(value)
        elif key == "author":
            meta["author"] = _unquote(value)
        elif key == "date":
            meta["date"] = _unquote(value)
        elif key == "updated":
            meta["updated"] = _unquote(value)
        elif key == "tags":
            meta["tags"] = _parse_tags(value)

    body = "\n".join(lines[i + 1 :]).strip()  # skip the closing '---'

    created_at = _parse_date(meta["date"])
    if created_at is None:
        return None
    updated_at = _parse_date(meta["updated"]) if meta["updated"] else None

    return BlogPost(
        slug=slug,
        title=meta["title"],
        body=body,
        author=meta["author"],
        created_at=created_at,
        updated_at=updated_at,
        tags=meta["tags"],
    )


def _unquote(value):
    value = value.strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
        return value[1:-1]
    return value


def _parse_tags(value):
    value = value.strip()
    if value.startswith("[") and value.endswith("]"):
        value = value[1:-1]
    return [_unquote(tag.strip()) for tag in value.split(",") if tag.strip()]


def _parse_date(value):
    if not value:
        return None
    value = value.strip()
    iso = value[:-1] + "+00:00" if value.endswith("Z") else value
    dt = None
    try:
        dt = datetime.fromisoformat(iso)
    except ValueError:
        try:
            dt = datetime.strptime(value, "%Y-%m-%d")
        except ValueError:
            return None
    if dt.tzinfo is None:
        dt = dt.replace(tzinfo=timezone.utc)
    return dt


# Singleton used by the pages.
blog_db = BlogDb()
