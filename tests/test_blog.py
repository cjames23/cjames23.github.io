"""Tests for the markdown blog data layer (app/blog.py).

The browser bundles the posts into Pyodide's VFS; here we exercise the same
parsing/rendering logic on the real files (via the build manifest) and on a
synthetic post.
"""

import build
from app.blog import BlogDb, _parse_date, _parse_post, _parse_tags

SAMPLE = """---
title: Hello, World
author: Tester
date: 2025-01-02T03:04:05Z
tags: [alpha, beta, gamma]
---

# Heading

A paragraph with **bold** and `code`.

- one
- two
"""


def test_parse_front_matter():
    post = _parse_post("hello-world", SAMPLE)
    assert post.slug == "hello-world"
    assert post.title == "Hello, World"
    assert post.author == "Tester"
    assert post.tags == ["alpha", "beta", "gamma"]
    assert post.created_at.year == 2025 and post.created_at.month == 1


def test_render_html_client_side():
    post = _parse_post("hello-world", SAMPLE)
    rendered = post.html()
    assert "<h1" in rendered
    assert "<strong>bold</strong>" in rendered
    assert "<code>code</code>" in rendered
    assert "<li>one</li>" in rendered


def test_excerpt_strips_markdown():
    post = _parse_post("hello-world", SAMPLE)
    excerpt = post.excerpt(40)
    assert "#" not in excerpt
    assert "**" not in excerpt
    assert excerpt.endswith("…")


def test_parse_tags_and_date_helpers():
    assert _parse_tags("[a, b]") == ["a", "b"]
    assert _parse_tags("c, d") == ["c", "d"]
    assert _parse_date("2025-03-22T21:30:00Z").tzinfo is not None
    assert _parse_date("2025-03-22").day == 22
    assert _parse_date("") is None


def test_missing_front_matter_returns_none():
    assert _parse_post("x", "no front matter here") is None


def test_blogdb_loads_real_posts_and_adjacency():
    # Ensure the manifest the browser reads is present, then load like the app does.
    build.write_index(build.discover_slugs())
    db = BlogDb()

    slugs = [p.slug for p in db.all()]
    assert "philosophy-of-slavoj-zizek" in slugs

    # Sorted newest-first.
    dates = [p.created_at for p in db.all()]
    assert dates == sorted(dates, reverse=True)

    newer, older = db.adjacent("philosophy-of-slavoj-zizek")
    assert newer is not None and older is not None

    # The oldest post has no "older" neighbour.
    oldest = db.all()[-1].slug
    assert db.adjacent(oldest)[1] is None
