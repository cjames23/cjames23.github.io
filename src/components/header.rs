// src/components/header.rs
use crate::app::ThemeContext;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Header)]
pub fn header() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("ThemeContext not found");
    let dark_mode = theme_context.dark_mode;

    // Define color palette based on theme
    let bg_header = if dark_mode {
        "bg-[#4F6F52]"
    } else {
        "bg-[#739072]"
    };
    let text_color = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#ECE3CE]"
    };
    let border_color = if dark_mode {
        "border-[#3A4D39]"
    } else {
        "border-[#4F6F52]"
    };

    html! {
        <div class={classes!(
            "flex",
            "items-center",
            "justify-between",
            "p-4",
            bg_header,
            text_color,
            "border-b",
            border_color,
            "shadow-sm"
        )}>
            <div class="flex items-center">
                <img src="logo.svg" alt="Logo" class="h-10 w-10 mr-3" />
                <h1 class="text-xl font-bold">{"C. James Hawkins"}</h1>
            </div>
            <div class="flex items-center space-x-4">
                // Additional header elements can go here
            </div>
        </div>
    }
}
