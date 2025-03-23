// src/components/nav.rs
use crate::app::ThemeContext;
use crate::components::nav_context::NavContext;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_context = use_context::<NavContext>().expect("NavContext not found");
    let theme_context = use_context::<ThemeContext>().expect("ThemeContext not found");

    let collapsed = nav_context.collapsed;

    let toggle_collapsed = {
        let toggle = nav_context.toggle_collapsed.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    // Convert toggle_theme Callback<()> to Callback<MouseEvent>
    let toggle_theme = {
        let theme_toggle = theme_context.toggle_theme.clone();
        Callback::from(move |_: MouseEvent| theme_toggle.emit(()))
    };

    let dark_mode = theme_context.dark_mode;

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
    let text_secondary = if dark_mode {
        "text-[#739072]"
    } else {
        "text-[#4F6F52]"
    };
    let hover_bg = if dark_mode {
        "hover:bg-[#5d8361]"
    } else {
        "hover:bg-[#86a684]"
    };

    let active_route = nav_context.active_route.clone();

    // Set the active route whenever the location changes
    let location = use_location().unwrap();
    let route_str = location.path().to_string();

    {
        let set_active_route = nav_context.set_active_route.clone();
        use_effect_with(route_str.clone(), move |route| {
            set_active_route.emit(route.clone());
            || ()
        });
    }

    html! {
        <nav class={classes!(
            "transition-all", "duration-300", "ease-in-out",
            bg_secondary,
            if collapsed { "w-16" } else { "w-64" },
            "fixed", "h-screen", "overflow-y-auto", "z-10"
        )}>
            <div class="flex flex-col h-full">
                <div class={classes!("flex", "items-center", "p-4", "border-b", "border-opacity-20", "border-slate-400")}>
                    if !collapsed {
                        <span class={classes!("font-bold", "text-xl", text_primary)}>{"Portfolio"}</span>
                    }
                    <button
                        class={classes!("ml-auto", text_primary, "hover:opacity-80", "transition-all")}
                        onclick={toggle_collapsed}
                    >
                        if collapsed {
                            <i class="fas fa-chevron-right"></i>
                        } else {
                            <i class="fas fa-chevron-left"></i>
                        }
                    </button>
                </div>

                <div class="flex flex-col py-4">
                    <NavLink
                        to={Route::Home}
                        classes={get_link_classes(&active_route, "/", collapsed,
               if dark_mode { "text-[#ECE3CE]" } else { "text-[#3A4D39]" },
               if dark_mode { "hover:bg-[#5d8361]" } else { "hover:bg-[#86a684]" })}
                        icon="fas fa-home"
                        label="Home"
                        {collapsed}
                    />
                    <NavLink
                        to={Route::Projects}
                        classes={get_link_classes(&active_route, "/", collapsed,
               if dark_mode { "text-[#ECE3CE]" } else { "text-[#3A4D39]" },
               if dark_mode { "hover:bg-[#5d8361]" } else { "hover:bg-[#86a684]" })}
                        icon="fas fa-code"
                        label="Projects"
                        {collapsed}
                    />
                    <NavLink
                        to={Route::Blog}
                        classes={get_link_classes(&active_route, "/", collapsed,
               if dark_mode { "text-[#ECE3CE]" } else { "text-[#3A4D39]" },
               if dark_mode { "hover:bg-[#5d8361]" } else { "hover:bg-[#86a684]" })}
                        icon="fas fa-blog"
                        label="Blog"
                        {collapsed}
                    />
                    <NavLink
                        to={Route::Contact}
                        classes={get_link_classes(&active_route, "/", collapsed,
               if dark_mode { "text-[#ECE3CE]" } else { "text-[#3A4D39]" },
               if dark_mode { "hover:bg-[#5d8361]" } else { "hover:bg-[#86a684]" })}
                        icon="fas fa-envelope"
                        label="Contact"
                        {collapsed}
                    />
                    <NavLink
                        to={Route::Admin}
                        classes={get_link_classes(&active_route, "/", collapsed,
               if dark_mode { "text-[#ECE3CE]" } else { "text-[#3A4D39]" },
               if dark_mode { "hover:bg-[#5d8361]" } else { "hover:bg-[#86a684]" })}                        icon="fas fa-lock"
                        label="Admin"
                        {collapsed}
                    />
                </div>

                <div class="mt-auto mb-4 px-4">
                    <button
                        onclick={toggle_theme}
                        class={classes!(
                            "flex", "items-center", "p-3", "rounded-md", "w-full",
                            text_primary, hover_bg, "transition-all"
                        )}
                    >
                        <i class={if dark_mode {"fas fa-sun"} else {"fas fa-moon"}}></i>
                        if !collapsed {
                            <span class="ml-3">
                                {if dark_mode {"Light Mode"} else {"Dark Mode"}}
                            </span>
                        }
                    </button>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
struct NavLinkProps {
    to: Route,
    classes: Classes,
    icon: &'static str,
    label: &'static str,
    collapsed: bool,
}

#[function_component(NavLink)]
fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <Link<Route> to={props.to.clone()} classes={props.classes.clone()}>
            <i class={props.icon}></i>
            if !props.collapsed {
                <span class="ml-3">{props.label}</span>
            }
        </Link<Route>>
    }
}

fn get_link_classes(
    active_route: &str,
    path: &str,
    collapsed: bool,
    text_color: &'static str,
    hover_bg: &'static str,
) -> Classes {
    classes!(
        "flex",
        "items-center",
        if collapsed {
            "justify-center"
        } else {
            "justify-start"
        },
        "p-3",
        "my-1",
        "mx-2",
        "rounded-md",
        "transition-all",
        text_color,
        hover_bg,
        if active_route == path {
            "bg-opacity-20 bg-white"
        } else {
            ""
        }
    )
}
