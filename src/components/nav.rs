// src/components/nav.rs
use crate::app::{Route, ThemeContext};
use gloo::events::EventListener;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::nav_context::NavContext;

#[function_component(Nav)]
pub fn nav() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let nav_ref = use_node_ref();
    let nav_context = use_context::<NavContext>().expect("NavContext not found");
    let collapsed = nav_context.collapsed; // This is already a bool, no need to dereference

    let toggle_collapsed = {
        let toggle = nav_context.toggle_collapsed.clone();
        Callback::from(move |_| toggle.emit(()))
    };
    // Toggle theme
    let toggle_theme = {
        let toggle_theme = theme_context.toggle_theme.clone();
        Callback::from(move |_| toggle_theme.emit(()))
    };

    // Menu items
    let menu_items = vec![
        (Route::Home, "Home", "home"),
        (Route::Projects, "Projects", "code"),
        (Route::Blog, "Blog", "book"),
        (Route::Contact, "Contact Me", "envelope"),
    ];

    let sidebar_width = if collapsed { "w-16" } else { "w-64" };
    let icon_transform = if collapsed { "" } else { "rotate-180" };

    html! {
        <div ref={nav_ref.clone()} class={classes!("sidebar", sidebar_width, "transition-all", "duration-300", "ease-in-out",
            "h-screen", "fixed", "top-0", "left-0", "z-40", "flex", "flex-col",
            "bg-gray-800", "dark:bg-gray-900", "text-white", "shadow-lg")}>

            // Logo and collapse button
            <div class="flex items-center justify-between p-4 border-b border-gray-700">
                <div class={classes!("logo", if collapsed { "hidden" } else { "" })}>
                    <h1 class="text-xl font-bold">{"Cary Hawkins"}</h1>
                </div>
                <button
                    onclick={toggle_collapsed}
                    class="p-2 rounded-full hover:bg-gray-700 focus:outline-none"
                    title="Toggle sidebar"
                >
                    <svg class={classes!("w-6", "h-6", "transition-transform", icon_transform)}
                         xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
                    </svg>
                </button>
            </div>

            // Navigation menu
            <div class="flex flex-col flex-grow py-4 overflow-y-auto">
                {
                    menu_items.into_iter().map(|(route, label, icon)| {
                        let to = route.clone();
                        html! {
                            <Link<Route> to={to} classes={classes!(
                                "flex", "items-center", "px-4", "py-3", "text-lg",
                                "hover:bg-gray-700", "transition-colors", "duration-200",
                                "focus:outline-none", "focus:bg-gray-700",
                            )}>
                                <span class="inline-flex items-center justify-center w-8 h-8">
                                    <i class={format!("fas fa-{}", icon)}></i>
                                </span>
                                if !collapsed {
                                    <span class="ml-3">{label}</span>
                                }
                            </Link<Route>>
                        }
                    }).collect::<Html>()
                }
            </div>

            // Theme toggle and user section
            <div class="p-4 border-t border-gray-700">
                <button
                    onclick={toggle_theme}
                    class="flex items-center justify-center w-full p-2 text-sm font-medium rounded-md hover:bg-gray-700 focus:outline-none"
                >
                    if theme_context.dark_mode {
                        <i class="fas fa-sun mr-2"></i>
                        if !collapsed {
                            <span>{"Light Mode"}</span>
                        }
                    } else {
                        <i class="fas fa-moon mr-2"></i>
                        if !collapsed {
                            <span>{"Dark Mode"}</span>
                        }
                    }
                </button>
            </div>
        </div>
    }
}
