use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/home"]
    Home,
    #[to = "/"]
    Index,
}

pub type Link = RouterAnchor<AppRoute>;
