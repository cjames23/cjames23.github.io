use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/contact")]
    Contact,
    #[at("/admin/login")]
    Login,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}
