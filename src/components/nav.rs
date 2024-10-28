use patternfly_yew::prelude::*;
use std::convert::Into;
use yew::prelude::*;
use yew_sidebar::{MenuItem, Sidebar, SidebarProps};

#[function_component(Nav)]
pub fn nav_bar() -> Html {
    let toggle_icon_collapsed = html! {
        <Brand src="logo.svg" alt="Cary Hawkins" />
    };
    let toggle_icon_expanded = html! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke="white"
            class="m-3 w-6 h-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h16m-7 6h7"
            />
        </svg>
    };
    let menu_items = vec![
        MenuItem {
            icon: html! { Icon::Home.as_html() },
            text: "Home",
            link: "/",
            class: "",
            title: "",
            submenus: vec![],
        },
        MenuItem {
            icon: html! { Icon::Catalog.as_html() },
            text: "Blog",
            link: "/blog",
            class: "",
            title: "",
            submenus: vec![],
        },
        MenuItem {
            icon: html! {},
            text: "Projects",
            link: "/projects",
            class: "",
            title: "",
            submenus: vec![],
        },
        MenuItem {
            icon: html! {},
            text: "Contact",
            link: "/contact",
            class: "",
            title: "",
            submenus: vec![],
        },
    ];
    let sidebar_props = SidebarProps {
        fixed: false,
        sider_collapsed: true,
        menu_items: menu_items.clone(),
        toggle_icon_collapsed: toggle_icon_collapsed.clone(),
        toggle_icon_expanded: toggle_icon_expanded.clone(),
        width_collapsed: "w-36",
        width_expanded: "w-56",
        padding_collapsed: "p-5",
        padding_expanded: "p-4",
        display_collapsed: "hidden",
        display_expanded: "flex",
        justify_content: "flex-col",
        align_items: "items-start",
        height: "h-screen",
        background_color: "rounded border-2 bg-gray-700",
        font: "text-lg text-white",
        button_border_radius: "",
        button_background_color: "transparent",
        button_height: "h-16",
        logo_src: "",
        bottom_section: Default::default(),
        ..SidebarProps::default()
    };
    html! {
            <Sidebar ..sidebar_props />
    }
}
