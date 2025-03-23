// src/components/layout.rs
use crate::app::ThemeContext;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::nav::Nav;
use crate::components::nav_context::NavContext;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let nav_context = use_context::<NavContext>().expect("NavContext not found");
    let theme_context = use_context::<ThemeContext>().expect("ThemeContext not found");

    let dark_mode = theme_context.dark_mode;
    let collapsed = nav_context.collapsed;

    // Define color palette based on theme
    let bg_primary = if dark_mode {
        "bg-[#3A4D39]"
    } else {
        "bg-[#ECE3CE]"
    };
    let bg_secondary = if dark_mode {
        "bg-[#4F6F52]"
    } else {
        "bg-[#739072]"
    };
    let text_primary = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#3A4D39]"
    };

    html! {
        <div class={classes!("h-screen", "flex", "overflow-hidden", bg_primary, text_primary)}>
            <Nav />
            <div class={classes!(
                "flex-grow",
                "flex",
                "flex-col",
                "transition-all",
                "duration-300",
                if collapsed { "ml-16" } else { "ml-64" }
            )}>
                // Fixed header
                <Header />

                // Scrollable main content
                <main class="flex-grow overflow-y-auto">
                    {props.children.clone()}
                </main>

                // Fixed footer
                <Footer class={classes!(bg_secondary, text_primary)} />
            </div>
        </div>
    }
}
