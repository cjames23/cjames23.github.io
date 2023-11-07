use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}
