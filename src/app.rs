// src/app.rs
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::Layout;
use crate::components::nav_context::NavProvider;
use crate::components::themecontext::ThemeProvider;
use crate::pages::admin::Admin;
use crate::pages::blog::Blog;
use crate::pages::blog_post::BlogPost;
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::login::Login;
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
    html! {
        <BrowserRouter>
            <ThemeProvider>
                <NavProvider>
                    <Switch<Route> render={switch} />
                </NavProvider>
            </ThemeProvider>
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
        Route::Admin => html! { <Layout><Admin /></Layout> },
        Route::Login => html! { <Layout><Login /></Layout> },
        Route::NotFound => html! { <div class="not-found"><h1>{"404 - Not Found"}</h1></div> },
    }
}
