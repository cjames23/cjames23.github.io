use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::home::Home;
use crate::pages::projects::Projects;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::About => html! { <About /> },
        Route::Projects => html! { <Projects /> },
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
