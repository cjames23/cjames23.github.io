use crate::app::ThemeContext;
use crate::components::footer::Footer;
use crate::components::header::Header;
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

    let content_margin = if *collapsed { "ml-16" } else { "ml-64" };

    html! {
        <ContextProvider<NavContext> context={nav_context}>
            <div class={classes!(
                "flex", "flex-col", "h-screen",
                if dark_mode { "dark" } else { "" }
            )}>
                <Nav />
                <div class={classes!("transition-all", "duration-300", content_margin, "flex", "flex-col", "h-screen")}>
                    <Header />
                    <main class={classes!("flex-grow", "overflow-y-auto", "p-6", "bg-white", "dark:bg-gray-900")}>
                        <div class="container mx-auto">
                            { for props.children.iter() }
                        </div>
                    </main>
                    <Footer/>
                </div>
            </div>
        </ContextProvider<NavContext>>
    }
}
