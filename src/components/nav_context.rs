// src/components/nav_context.rs
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NavContext {
    pub active_route: String,
    pub set_active_route: Callback<String>,
    pub collapsed: bool,
    pub toggle_collapsed: Callback<()>,
}

#[function_component(NavProvider)]
pub fn nav_provider(props: &yew::html::ChildrenProps) -> Html {
    let active_route = use_state(|| "".to_string());
    let collapsed = use_state(|| false); // Define collapsed state

    let set_active_route = {
        let active_route = active_route.clone();
        Callback::from(move |route: String| {
            active_route.set(route);
        })
    };

    let toggle_collapsed = {
        let collapsed = collapsed.clone();
        Callback::from(move |_: ()| {
            collapsed.set(!*collapsed);
        })
    };

    let context = NavContext {
        active_route: (*active_route).clone(),
        set_active_route,
        collapsed: *collapsed,
        toggle_collapsed,
    };

    html! {
        <ContextProvider<NavContext> context={context}>
            {props.children.clone()}
        </ContextProvider<NavContext>>
    }
}
