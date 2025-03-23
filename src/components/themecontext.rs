use crate::app::ThemeAction;
use crate::app::ThemeContext;
use crate::app::ThemeState;
use crate::components::nav_context::NavProvider;
use web_sys;
use yew::function_component;
use yew::prelude::*;
use yew_router::prelude::Router;

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &yew::html::ChildrenProps) -> Html {
    let theme_state = use_reducer(|| ThemeState { dark_mode: false });

    let toggle_theme = {
        let theme_state = theme_state.clone();
        Callback::from(move |_| {
            theme_state.dispatch(ThemeAction::Toggle);

            // Update document class for CSS
            if let Some(document) = web_sys::window().and_then(|win| win.document()) {
                if let Some(html) = document.document_element() {
                    if theme_state.dark_mode {
                        let _ = html.class_list().add_1("dark");
                    } else {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }
        })
    };

    let context = ThemeContext {
        dark_mode: theme_state.dark_mode,
        toggle_theme,
    };

    html! {
        <ContextProvider<ThemeContext> context={context}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}
