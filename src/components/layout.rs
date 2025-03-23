// src/components/layout.rs
use crate::app::ThemeContext;
use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::components::nav_context::NavContext;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let collapsed = use_state(|| false);
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    let toggle_collapsed = {
        let collapsed = collapsed.clone();
        Callback::from(move |_| collapsed.set(!*collapsed))
    };

    let nav_context = NavContext {
        collapsed: *collapsed,
        toggle_collapsed,
    };

    // Change this line to use no margin when collapsed
    let main_margin = if *collapsed { "ml-0" } else { "ml-64" };

    html! {
        <ContextProvider<NavContext> context={nav_context}>
            <div class={classes!(
                "flex", "flex-col", "min-h-screen",
                if dark_mode { "dark" } else { "" }
            )}>
                <Nav />
                <main class={classes!("flex-grow", "transition-all", "duration-300", main_margin, "p-6", "bg-white", "dark:bg-gray-900")}>
                    <div class="container mx-auto">
                        { for props.children.iter() }
                    </div>
                </main>
                <Footer />
            </div>
        </ContextProvider<NavContext>>
    }
}
