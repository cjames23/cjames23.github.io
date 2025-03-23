use yew::prelude::*;
use yew_router::prelude::*;
use std::rc::Rc;

use crate::pages::blog::Blog;
use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::pages::contact::Contact;
use crate::components::layout::Layout;

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
            ThemeAction::Toggle => Self { dark_mode: !self.dark_mode }.into(),
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
    let theme_state = use_reducer(|| ThemeState { dark_mode: false });

    let toggle_theme = {
        let theme_state = theme_state.clone();
        Callback::from(move |_| theme_state.dispatch(ThemeAction::Toggle))
    };

    let theme_context = ThemeContext {
        dark_mode: theme_state.dark_mode,
        toggle_theme,
    };

    let theme_class = if theme_state.dark_mode { "dark" } else { "" };

    html! {
        <ContextProvider<ThemeContext> context={theme_context}>
            <div class={classes!("app-container", theme_class)}>
                <BrowserRouter>
                    <Layout>
                        <Switch<Route> render={switch} />
                    </Layout>
                </BrowserRouter>
            </div>
        </ContextProvider<ThemeContext>>
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Projects => html! { <Projects /> },
        Route::Blog => html! { <Blog /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <div class="not-found"><h1>{"404 - Not Found"}</h1></div> },
    }
}