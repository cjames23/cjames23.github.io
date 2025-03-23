// src/components/layout.rs
use crate::app::ThemeContext;
use crate::components::footer::Footer;
use crate::components::nav::Nav;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let theme_class = if theme_context.dark_mode { "dark" } else { "" };

    html! {
        <div class={classes!("layout", theme_class)}>
            <Nav />
            <main class="main-content">
                { for props.children.iter() }
            </main>
            <Footer />
        </div>
    }
}
