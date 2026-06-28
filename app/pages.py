"""Pages: blog list (home), single post, projects, about, 404."""

from puepy import Page, t
from puepy.core import html

from app.blog import blog_db
from app.components import link


def _tag(name):
    t.span(name, classes="tag")


class BasePage(Page):
    pass


def _register(app):
    """Attach all pages to the given Application."""

    @app.page()
    class HomePage(BasePage):
        def populate(self):
            with t.layout() as layout, layout.slot():
                t.h1("Writing", classes="page-title")
                t.p(
                    "Notes on software, philosophy, and the strange places where engineering meets theory.",
                    classes="page-intro",
                )
                with t.div(classes="post-list"):
                    for post in blog_db.all():
                        self.populate_card(post)

        def populate_card(self, post):
            with t.article(classes="card"):
                with t.div(classes="post-meta"):
                    t.span(t.i(classes="far fa-calendar-alt"), post.formatted_date)
                    t.span(t.i(classes="fas fa-user"), post.author)
                link(self, post.title, f"/blog/{post.slug}", classes="post-title-link")
                t.p(post.excerpt(), classes="post-excerpt")
                with t.div(classes="tags"):
                    for tag in post.tags:
                        _tag(tag)
                with t.div(classes="card-foot"):
                    link(
                        self,
                        ["Read article", t.i(classes="fas fa-arrow-right")],
                        f"/blog/{post.slug}",
                        classes="read-link",
                    )

    @app.page("/blog/<slug>")
    class BlogPostPage(BasePage):
        props = ["slug"]

        def populate(self):
            with t.layout() as layout, layout.slot():
                post = blog_db.by_slug(self.slug)
                if post is None:
                    self.populate_missing()
                else:
                    self.populate_post(post)

        def populate_post(self, post):
            newer, older = blog_db.adjacent(post.slug)

            link(
                self,
                [t.i(classes="fas fa-arrow-left"), "Back to all posts"],
                "/",
                classes="back-link",
            )

            with t.article(classes="card"):
                with t.header(classes="post-header"):
                    t.h1(post.title, classes="post-h1")
                    with t.div(classes="byline"):
                        t.span(t.i(classes="fas fa-user"), post.author)
                        t.span(t.i(classes="far fa-calendar-alt"), post.formatted_date)
                        t.span(t.i(classes="far fa-clock"), f"{post.reading_time} min read")
                    with t.div(classes="tags"):
                        for tag in post.tags:
                            _tag(tag)
                with t.section(classes="markdown-content"):
                    t(html(post.html()))

            self.populate_nav(newer, older)

        def populate_nav(self, newer, older):
            with t.nav(classes="postnav"):
                if newer:
                    link(
                        self,
                        [
                            t.span(
                                t.i(classes="fas fa-arrow-left"),
                                "Newer",
                                classes="postnav-label",
                            ),
                            t.span(newer.title, classes="postnav-title"),
                        ],
                        f"/blog/{newer.slug}",
                        classes="postnav-card",
                    )
                else:
                    t.span(classes="postnav-empty")

                link(
                    self,
                    [t.i(classes="fas fa-th-list"), "All posts"],
                    "/",
                    classes="postnav-card postnav-card--all",
                )

                if older:
                    link(
                        self,
                        [
                            t.span(
                                "Older",
                                t.i(classes="fas fa-arrow-right"),
                                classes="postnav-label",
                            ),
                            t.span(older.title, classes="postnav-title"),
                        ],
                        f"/blog/{older.slug}",
                        classes="postnav-card postnav-card--right",
                    )
                else:
                    t.span(classes="postnav-empty")

        def populate_missing(self):
            t.h1("Post not found", classes="page-title")
            t.p("That post doesn't exist.", classes="muted")
            link(self, "← Back to all posts", "/", classes="back-link")

    @app.page("/projects")
    class ProjectsPage(BasePage):
        PROJECTS = [
            (
                "Co-maintainer of Hatch",
                "Co-maintainer of Hatch, a modern Python project manager focused on "
                "extensibility and performance. Helped ship the long-awaited monorepo "
                "workspace support.",
                "https://github.com/pypa/hatch/pull/2073",
                "View the workspaces PR",
            ),
            (
                "Contributor to Apache Airflow",
                "Released the first pieces of an OpenSearch Airflow provider.",
                "https://www.linkedin.com/posts/apache-airflow_new-apache-airflow-provider-weve-just-released-activity-7122632024523218944-suuC",
                "Read the announcement",
            ),
            (
                "PyPI Proxy Poetry Plugin",
                "Maintain a Poetry plugin that handles proxy repositories for PyPI, including authentication.",
                "https://github.com/chadac/poetry-plugin-pypi-proxy",
                "View on GitHub",
            ),
            (
                "This website",
                "A personal site rebuilt to run entirely in the browser with Python — "
                "PyScript + Pyodide + the PuePy framework — rendering this very blog's "
                "markdown client-side.",
                "https://github.com/cjames23/cjames23.github.io",
                "View on GitHub",
            ),
        ]

        def populate(self):
            with t.layout() as layout, layout.slot():
                t.h1("Projects", classes="page-title")
                with t.div(classes="grid-2 section-gap"):
                    for title, body, url, cta in self.PROJECTS:
                        with t.div(classes="card"):
                            t.h2(title, classes="card-title")
                            t.p(body, classes="card-body")
                            t.a(
                                cta,
                                t.i(classes="fas fa-arrow-up-right-from-square"),
                                href=url,
                                classes="project-link",
                            )

    @app.page("/about")
    class AboutPage(BasePage):
        BIO = (
            "I have a rather unorthodox background as an engineer. I studied Philosophy "
            "in undergrad and pursued a life in the kitchen as a chef for over 15 years "
            "before turning towards tech. I have now been in the space of Data and "
            "Software since 2014 — starting as a Metrics Analyst at a small startup in "
            "Phoenix, AZ, where I eventually became a database administrator before "
            "becoming a Data Engineer at Amazon in 2017. My passions keep evolving; "
            "lately I'm drawn to developer experience and build tools."
        )
        SKILLS = [
            "Rust",
            "Python",
            "AWS",
            "Database Design",
            "Data Engineering",
            "PyO3",
            "Maturin",
            "PyScript",
            "Hatch",
            "Python Packaging"
        ]

        def populate(self):
            with t.layout() as layout, layout.slot():
                t.h1(
                    "I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA.",
                    classes="about-h1",
                )
                t.p(self.BIO, classes="about-bio")
                with t.div(classes="grid-2 section-gap"):
                    with t.div(classes="card"):
                        t.h2("Skills", classes="card-title")
                        with t.div(classes="skills"):
                            for skill in self.SKILLS:
                                t.span(skill, classes="skill")
                    with t.div(classes="card"):
                        t.h2("Recent Projects", classes="card-title")
                        with t.ul(classes="list"):
                            t.li("Personal portfolio site (Python / PyScript / PuePy)")
                            t.li("Hatch lockfile support and default type checking with pyrefly")
                            t.li("OpenSearch Airflow provider")

    @app.page("/404")
    class NotFoundPage(BasePage):
        props = ["error"]

        def populate(self):
            with t.layout() as layout, layout.slot():
                t.h1("404", classes="notfound-code")
                t.p("That page doesn't exist.", classes="muted")
                link(
                    self,
                    [t.i(classes="fas fa-arrow-left"), "Back home"],
                    "/",
                    classes="back-link",
                )

    return NotFoundPage
