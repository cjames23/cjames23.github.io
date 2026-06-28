"""Shared UI: the app shell (collapsible sidebar + header + footer), the
SPA-aware link helper, and dark-mode handling. Styling lives in style.css."""

import js
from puepy import Component, t

NAV_ITEMS = [
    ("Home", "fa-house", "/"),
    ("Projects", "fa-code", "/projects"),
    ("About", "fa-user", "/about"),
]


def apply_theme(dark):
    """Toggle the `dark` class on <html> so the CSS dark-mode variables engage."""
    class_list = js.document.documentElement.classList
    if dark:
        class_list.add("dark")
    else:
        class_list.remove("dark")


def link(owner, content, path, **attrs):
    """An <a> that navigates via the SPA router instead of a full page load.

    `owner` is any page/component (it exposes `.application`). `content` may be a
    string, a single tag, or a list/tuple of children (passed as positional args).
    """
    router = owner.application.router

    def on_click(event):
        event.preventDefault()
        router.navigate_to_path(path)

    if isinstance(content, (list, tuple)):
        return t.a(*content, href=path, on_click=on_click, **attrs)
    return t.a(content, href=path, on_click=on_click, **attrs)


@t.component()
class Layout(Component):
    def populate(self):
        collapsed = self.application.state["sidebar_collapsed"]
        with t.div(classes="app"):
            self.populate_sidebar(collapsed)
            with t.div(classes="content"):
                with t.main(classes="main"), t.div(classes="container"):
                    self.insert_slot()
                self.populate_footer()

    def populate_sidebar(self, collapsed):
        with t.aside(classes="sidebar sidebar--collapsed" if collapsed else "sidebar"):
            with t.div(classes="sidebar-head"):
                if not collapsed:
                    link(self, "C. James Hawkins", "/", classes="brand")
                t.button(
                    t.i(classes=f"fas {'fa-chevron-right' if collapsed else 'fa-chevron-left'}"),
                    classes="icon-btn",
                    on_click=self.toggle_sidebar,
                    title="Toggle sidebar",
                )

            with t.nav(classes="nav"):
                current = self.application.current_path or "/"
                for label, icon, path in NAV_ITEMS:
                    active = current == path or (path != "/" and current.startswith(path))
                    link(
                        self,
                        [
                            t.i(classes=f"fas {icon}"),
                            None if collapsed else t.span(label),
                        ],
                        path,
                        classes="nav-link nav-link--active" if active else "nav-link",
                        title=label,
                    )

            with t.div(classes="nav-foot"):
                dark = self.application.state["dark_mode"]
                t.button(
                    t.i(classes=f"fas {'fa-sun' if dark else 'fa-moon'}"),
                    None if collapsed else t.span("Light mode" if dark else "Dark mode"),
                    classes="nav-link",
                    on_click=self.toggle_theme,
                    title="Toggle dark mode",
                )

    def populate_footer(self):
        with t.footer(classes="footer"):
            t.span("© 2025 C. James Hawkins")
            with t.span(classes="footer-links"):
                t.a(t.i(classes="fab fa-github"), href="https://github.com/cjames23", title="GitHub")
                t.a(
                    t.i(classes="fab fa-linkedin"),
                    href="https://www.linkedin.com/in/cary-hawkins",
                    title="LinkedIn",
                )

    # -- handlers ---------------------------------------------------------
    def toggle_sidebar(self, event):
        state = self.application.state
        state["sidebar_collapsed"] = not state["sidebar_collapsed"]
        # Force a clean rebuild: an in-place reactive redraw re-renders children
        # but doesn't re-apply the sidebar element's own width class.
        self.application.remount()

    def toggle_theme(self, event):
        state = self.application.state
        dark = not state["dark_mode"]
        state["dark_mode"] = dark
        apply_theme(dark)
        storage = self.application.local_storage
        if storage is not None:
            storage["dark_mode"] = "1" if dark else "0"
