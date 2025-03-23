use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::Layout;
use crate::pages::blog::Blog;
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::projects::Projects;

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

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/blog")]
    Blog,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
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

    let toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let new_value = !*dark_mode;
            if let Some(storage) = web_sys::window()
                .and_then(|win| win.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item("darkMode", &new_value.to_string());
            }
            dark_mode.set(new_value);
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
        Route::NotFound => html! { <div class="not-found"><h1>{"404 - Not Found"}</h1></div> },
    }
}
