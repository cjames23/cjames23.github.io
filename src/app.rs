use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::Layout;
use crate::pages::blog::Blog;
use crate::pages::blog_post::BlogPost;
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::route::Route;

// Theme context
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContext {
    pub dark_mode: bool,
    pub toggle_theme: Callback<()>,
}

pub type ThemeContextHandle = UseReducerHandle<ThemeState>;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeState {
    pub dark_mode: bool,
}

pub enum ThemeAction {
    Toggle,
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ThemeAction::Toggle => Self {
                dark_mode: !self.dark_mode,
            }
            .into(),
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let dark_mode = use_state(|| {
        // Check local storage or system preference for initial value
        web_sys::window()
            .and_then(|win| win.local_storage().ok())
            .flatten()
            .and_then(|storage| storage.get_item("darkMode").ok())
            .flatten()
            .map(|val| val == "true")
            .unwrap_or(false)
    });

    let dark_mode = use_state(|| true); // Initial state, true for dark mode
    let toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let new_mode = !*dark_mode;
            dark_mode.set(new_mode);

            // Update the document class to reflect the theme change
            if let Some(document) = web_sys::window().and_then(|win| win.document()) {
                if let Some(html) = document.document_element() {
                    if new_mode {
                        let _ = html.class_list().add_1("dark");
                    } else {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }
        })
    };

    let theme_context = ThemeContext {
        dark_mode: *dark_mode,
        toggle_theme,
    };

    html! {
        <BrowserRouter>
            <ContextProvider<ThemeContext> context={theme_context}>
                <div class={classes!("min-h-screen", "transition-colors", "duration-300",
                               if *dark_mode { "dark" } else { "" })}>
                    <Switch<Route> render={switch} />
                </div>
            </ContextProvider<ThemeContext>>
        </BrowserRouter>
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Layout><Home /></Layout> },
        Route::Contact => html! { <Layout><Contact /></Layout> },
        Route::Projects => html! { <Layout><Projects /></Layout> },
        Route::Blog => html! { <Layout><Blog /></Layout> },
        Route::BlogPost { id } => html! { <Layout><BlogPost {id} /></Layout> },
        Route::NotFound => html! { <div class="not-found"><h1>{"404 - Not Found"}</h1></div> },
    }
}
