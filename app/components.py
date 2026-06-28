"""Shared UI: the app shell (collapsible sidebar + header + footer), the
SPA-aware link helper, and dark-mode handling."""

import js
from puepy import Component, t

# Midnight Python palette (Tailwind arbitrary values, light / dark via the `dark:` variant)
SIDEBAR_BG = "bg-white dark:bg-[#1E293B]"
CARD_BG = "bg-white dark:bg-[#1E293B]"
CARD_TEXT = "text-[#0F172A] dark:text-[#E2E8F0]"
BORDER = "border-[#E2E8F0] dark:border-[#334155]"
SUBTLE = "text-[#475569] dark:text-[#94A3B8]"
ACCENT = "text-[#2563EB] dark:text-[#38BDF8]"  # electric-blue links / CTAs

NAV_ITEMS = [
    ("Home", "fa-house", "/"),
    ("Projects", "fa-code", "/projects"),
    ("About", "fa-user", "/about"),
]


def apply_theme(dark):
    """Toggle the `dark` class on <html> so Tailwind's dark: variants engage."""
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
        with t.div(classes="flex h-screen overflow-hidden"):
            self.populate_sidebar(collapsed)
            with t.div(classes="flex-1 flex flex-col min-w-0"):
                with t.main(classes="flex-1 overflow-y-auto"):
                    with t.div(classes="mx-auto w-full max-w-4xl px-5 py-10 md:px-8"):
                        self.insert_slot()
                self.populate_footer()

    def populate_sidebar(self, collapsed):
        width = "w-16" if collapsed else "w-64"
        with t.aside(
            classes=f"{SIDEBAR_BG} {width} shrink-0 h-screen flex flex-col "
            "transition-all duration-300 ease-in-out border-r border-[#E2E8F0] dark:border-[#334155]"
        ):
            # Brand / collapse toggle
            with t.div(classes="flex items-center gap-2 p-4 border-b border-[#E2E8F0] dark:border-[#334155]"):
                if not collapsed:
                    link(
                        self,
                        "C. James Hawkins",
                        "/",
                        classes="font-semibold tracking-tight text-lg truncate",
                    )
                t.button(
                    t.i(classes=f"fas {'fa-chevron-right' if collapsed else 'fa-chevron-left'}"),
                    classes="ml-auto p-2 rounded-md hover:bg-black/10 dark:hover:bg-white/10 transition",
                    on_click=self.toggle_sidebar,
                    title="Toggle sidebar",
                )

            # Nav
            with t.nav(classes="flex flex-col gap-1 p-3"):
                current = self.application.current_path or "/"
                for label, icon, path in NAV_ITEMS:
                    active = current == path or (path != "/" and current.startswith(path))
                    active_cls = (
                        "bg-[#2563EB]/10 dark:bg-[#38BDF8]/15 text-[#2563EB] dark:text-[#38BDF8] font-semibold"
                        if active
                        else ""
                    )
                    link(
                        self,
                        [
                            t.i(classes=f"fas {icon} w-5 text-center"),
                            None if collapsed else t.span(label, classes="ml-1"),
                        ],
                        path,
                        classes=f"flex items-center gap-2 px-3 py-2 rounded-lg "
                        f"hover:bg-black/5 dark:hover:bg-white/10 transition {active_cls}"
                        + (" justify-center" if collapsed else ""),
                        title=label,
                    )

            # Dark-mode toggle
            with t.div(classes="mt-auto p-3"):
                dark = self.application.state["dark_mode"]
                t.button(
                    t.i(classes=f"fas {'fa-sun' if dark else 'fa-moon'} w-5 text-center"),
                    None if collapsed else t.span("Light mode" if dark else "Dark mode", classes="ml-1"),
                    classes="flex items-center gap-2 w-full px-3 py-2 rounded-lg "
                    "hover:bg-black/5 dark:hover:bg-white/10 transition" + (" justify-center" if collapsed else ""),
                    on_click=self.toggle_theme,
                    title="Toggle dark mode",
                )

    def populate_footer(self):
        with t.footer(
            classes="px-6 py-4 text-sm border-t border-[#E2E8F0] dark:border-[#334155] "
            "flex flex-wrap items-center justify-between gap-3"
        ):
            t.span("© 2025 C. James Hawkins", classes=SUBTLE)
            with t.span(classes="flex items-center gap-4"):
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
