"""Entry point: build the PuePy application, restore theme, mount, hide splash."""

import sys

sys.path.insert(0, ".")

from puepy import Application
from puepy.router import Router

from app.components import apply_theme  # importing this also registers the Layout component
from app.pages import _register


class SiteApp(Application):
    def initial(self):
        return {"dark_mode": False, "sidebar_collapsed": False}


app = SiteApp()
app.install_router(Router, link_mode=Router.LINK_MODE_HTML5)
app.not_found_page = _register(app)

# Restore persisted dark-mode preference before the first render.
storage = app.local_storage
dark = storage is not None and storage.get("dark_mode") == "1"
app.state["dark_mode"] = dark
apply_theme(dark)

app.mount("#app")

# Remove the boot splash now that Python has taken over.
from js import document

splash = document.getElementById("loading-splash")
if splash:
    splash.remove()
