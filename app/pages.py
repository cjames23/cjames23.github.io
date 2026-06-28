"""Pages: blog list (home), single post, projects, about, 404."""

from puepy import Page, t
from puepy.core import html

from app.blog import blog_db
from app.components import BORDER, CARD_BG, CARD_TEXT, SUBTLE, link

TAG_COLORS = {
    "rust": "bg-red-700",
    "yew": "bg-blue-700",
    "python": "bg-amber-600",
    "ai": "bg-cyan-700",
    "ethics": "bg-rose-700",
    "obligation": "bg-rose-800",
    "deconstruction": "bg-fuchsia-800",
    "philosophy": "bg-indigo-700",
    "zizek": "bg-violet-800",
    "lacan": "bg-purple-800",
}


def _tag(name):
    color = TAG_COLORS.get(name.lower(), "bg-black/30")
    t.span(name, classes=f"{color} text-white px-2 py-0.5 rounded text-xs tracking-wide")


def _card_classes(*extra):
    return " ".join([CARD_BG, CARD_TEXT, "rounded-xl shadow-md border", BORDER, *extra])


class BasePage(Page):
    pass


def _register(app):
    """Attach all pages to the given Application."""

    @app.page()
    class HomePage(BasePage):
        def populate(self):
            with t.layout() as layout:
                with layout.slot():
                    t.h1("Writing", classes="text-4xl font-bold tracking-tight")
                    t.p(
                        "Notes on software, philosophy, and the strange places where engineering meets theory.",
                        classes=f"mt-2 mb-8 max-w-2xl {SUBTLE}",
                    )
                    with t.div(classes="flex flex-col gap-6"):
                        for post in blog_db.all():
                            self.populate_card(post)

        def populate_card(self, post):
            with t.article(classes=_card_classes("p-6 md:p-8")):
                with t.div(classes="flex flex-wrap items-center gap-3 text-sm opacity-80"):
                    t.span(t.i(classes="far fa-calendar-alt mr-2"), post.formatted_date)
                    t.span(t.i(classes="fas fa-user mr-2"), post.author)
                link(
                    self,
                    post.title,
                    f"/blog/{post.slug}",
                    classes="block mt-3 text-2xl md:text-3xl font-bold leading-tight hover:underline",
                )
                t.p(post.excerpt(), classes="mt-3 leading-7 opacity-95")
                with t.div(classes="mt-4 flex flex-wrap gap-2"):
                    for tag in post.tags:
                        _tag(tag)
                with t.div(classes=f"mt-5 pt-4 border-t {BORDER} flex justify-end"):
                    link(
                        self,
                        ["Read article", t.i(classes="fas fa-arrow-right ml-2")],
                        f"/blog/{post.slug}",
                        classes="inline-flex items-center font-medium hover:underline",
                    )

    @app.page("/blog/<slug>")
    class BlogPostPage(BasePage):
        props = ["slug"]

        def populate(self):
            with t.layout() as layout:
                with layout.slot():
                    post = blog_db.by_slug(self.slug)
                    if post is None:
                        self.populate_missing()
                    else:
                        self.populate_post(post)

        def populate_post(self, post):
            newer, older = blog_db.adjacent(post.slug)

            with t.div(classes=f"text-sm mb-6 {SUBTLE}"):
                link(self, "← Back to all posts", "/", classes="hover:underline")

            with t.article(classes=_card_classes("p-6 md:p-10")):
                with t.header(classes=f"pb-6 mb-6 border-b {BORDER}"):
                    t.h1(post.title, classes="text-3xl md:text-4xl font-bold leading-tight")
                    with t.div(classes="mt-4 flex flex-wrap items-center gap-x-4 gap-y-2 text-sm opacity-80"):
                        t.span(t.i(classes="fas fa-user mr-2"), post.author)
                        t.span(t.i(classes="far fa-calendar-alt mr-2"), post.formatted_date)
                        t.span(t.i(classes="far fa-clock mr-2"), f"{post.reading_time} min read")
                    with t.div(classes="mt-4 flex flex-wrap gap-2"):
                        for tag in post.tags:
                            _tag(tag)
                with t.section(classes="markdown-content text-base md:text-lg leading-8"):
                    t(html(post.html()))

            self.populate_nav(newer, older)

        def populate_nav(self, newer, older):
            with t.nav(classes=f"mt-8 pt-6 border-t {BORDER} grid grid-cols-1 sm:grid-cols-3 gap-4"):
                if newer:
                    link(
                        self,
                        [
                            t.span(
                                t.i(classes="fas fa-arrow-left mr-2"),
                                "Newer",
                                classes="text-xs uppercase tracking-wide opacity-70",
                            ),
                            t.span(newer.title, classes="font-medium mt-1 block"),
                        ],
                        f"/blog/{newer.slug}",
                        classes=_card_classes("p-4 hover:opacity-90 sm:text-left"),
                    )
                else:
                    t.span(classes="hidden sm:block")

                link(
                    self,
                    [t.i(classes="fas fa-th-list mr-2"), "All posts"],
                    "/",
                    classes=_card_classes("p-4 flex items-center justify-center hover:opacity-90"),
                )

                if older:
                    link(
                        self,
                        [
                            t.span(
                                "Older",
                                t.i(classes="fas fa-arrow-right ml-2"),
                                classes="text-xs uppercase tracking-wide opacity-70",
                            ),
                            t.span(older.title, classes="font-medium mt-1 block"),
                        ],
                        f"/blog/{older.slug}",
                        classes=_card_classes("p-4 hover:opacity-90 sm:text-right"),
                    )
                else:
                    t.span(classes="hidden sm:block")

        def populate_missing(self):
            t.h1("Post not found", classes="text-3xl font-bold")
            t.p("That post doesn't exist.", classes=f"mt-2 {SUBTLE}")
            link(self, "← Back to all posts", "/", classes="inline-block mt-4 hover:underline")

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
                "https://github.com/cjames23/portfolio",
                "View on GitHub",
            ),
        ]

        def populate(self):
            with t.layout() as layout:
                with layout.slot():
                    t.h1("Projects", classes="text-4xl font-bold tracking-tight mb-8")
                    with t.div(classes="grid grid-cols-1 md:grid-cols-2 gap-6"):
                        for title, body, url, cta in self.PROJECTS:
                            with t.div(classes=_card_classes("p-6")):
                                t.h2(title, classes="text-xl font-bold")
                                t.p(body, classes="mt-2 leading-7 opacity-95")
                                t.a(
                                    cta,
                                    t.i(classes="fas fa-arrow-up-right-from-square ml-2"),
                                    href=url,
                                    classes="inline-flex items-center mt-4 font-medium underline hover:opacity-90",
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
            "Java",
            "AWS",
            "Database Design",
            "Data Engineering",
            "Gradle",
            "Kotlin",
            "Scala",
            "PyO3",
            "Maturin",
            "PyScript",
        ]

        def populate(self):
            with t.layout() as layout:
                with layout.slot():
                    t.h1(
                        "I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA.",
                        classes="text-3xl md:text-4xl font-bold tracking-tight",
                    )
                    t.p(self.BIO, classes="mt-6 leading-8 max-w-3xl")
                    with t.div(classes="grid grid-cols-1 md:grid-cols-2 gap-6 mt-10"):
                        with t.div(classes=_card_classes("p-6")):
                            t.h2("Skills", classes="text-xl font-bold mb-3")
                            with t.div(classes="flex flex-wrap gap-2"):
                                for skill in self.SKILLS:
                                    t.span(
                                        skill,
                                        classes="bg-black/15 dark:bg-white/10 px-3 py-1 rounded text-sm",
                                    )
                        with t.div(classes=_card_classes("p-6")):
                            t.h2("Recent Projects", classes="text-xl font-bold mb-3")
                            with t.ul(classes="list-disc list-inside space-y-2"):
                                t.li("Personal portfolio site (Python / PyScript / PuePy)")
                                t.li("Hatch monorepo workspace support")
                                t.li("OpenSearch Airflow provider")

    @app.page("/404")
    class NotFoundPage(BasePage):
        props = ["error"]

        def populate(self):
            with t.layout() as layout:
                with layout.slot():
                    t.h1("404", classes="text-5xl font-bold")
                    t.p("That page doesn't exist.", classes=f"mt-2 {SUBTLE}")
                    link(self, "← Back home", "/", classes="inline-block mt-4 hover:underline")

    return NotFoundPage
