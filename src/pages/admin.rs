// src/pages/admin.rs
use crate::app::ThemeContext;
use crate::route::Route;
use gloo::storage::{LocalStorage, Storage};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Admin)]
pub fn admin() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;
    let navigator = use_navigator().unwrap();

    // Check if user is logged in first
    let auth_token = LocalStorage::get::<String>("auth_token").unwrap_or_default();
    let is_logged_in = !auth_token.is_empty();

    // Clone navigator for the effect
    let navigator_for_effect = navigator.clone();

    // Then use it in the effect
    use_effect_with(is_logged_in, move |is_logged_in| {
        if !*is_logged_in {
            navigator_for_effect.push(&Route::Login);
        }
        || ()
    });

    // Define color palette
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
    let border_color = if dark_mode {
        "border-[#739072]"
    } else {
        "border-[#4F6F52]"
    };

    let logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let _ = LocalStorage::delete("auth_token");
            navigator.push(&Route::Login);
        })
    };

    if !is_logged_in {
        // Return empty while redirecting
        return html! {};
    }

    html! {
        <div class={classes!("min-h-screen", bg_primary, text_primary)}>
            // Header with logout
            <header class={classes!("w-full", bg_secondary, "shadow-md")}>
                <div class="container mx-auto px-4">
                    <div class="flex justify-between items-center h-16">
                        <div class="flex items-center">
                            <span class={classes!("text-xl", "font-bold", text_primary)}>{"Admin Dashboard"}</span>
                        </div>

                        <div class="flex items-center">
                            <button
                                onclick={logout}
                                class={classes!(
                                    "flex", "items-center", "px-3", "py-2", "rounded-md",
                                    bg_primary, text_secondary, "hover:opacity-90"
                                )}>
                                <i class="fas fa-sign-out-alt mr-2"></i>
                                {"Logout"}
                            </button>
                        </div>
                    </div>
                </div>
            </header>

            <main class="container mx-auto px-4 py-8">
                <div class={classes!("grid", "grid-cols-1", "md:grid-cols-3", "gap-6")}>

                    <div class={classes!(bg_secondary, "rounded-lg", "shadow", "p-6", text_primary)}>
                        <div class="flex justify-between items-center">
                            <div>
                                <p class="text-sm opacity-70">{"Total Posts"}</p>
                                <h3 class="text-3xl font-bold">{"12"}</h3>
                            </div>
                            <div class={classes!(bg_primary, "p-3", "rounded-full")}>
                                <i class="fas fa-file-alt text-xl"></i>
                            </div>
                        </div>
                    </div>

                    <div class={classes!(bg_secondary, "rounded-lg", "shadow", "p-6", text_primary)}>
                        <div class="flex justify-between items-center">
                            <div>
                                <p class="text-sm opacity-70">{"Comments"}</p>
                                <h3 class="text-3xl font-bold">{"48"}</h3>
                            </div>
                            <div class={classes!(bg_primary, "p-3", "rounded-full")}>
                                <i class="fas fa-comments text-xl"></i>
                            </div>
                        </div>
                    </div>

                    <div class={classes!(bg_secondary, "rounded-lg", "shadow", "p-6", text_primary)}>
                        <div class="flex justify-between items-center">
                            <div>
                                <p class="text-sm opacity-70">{"Projects"}</p>
                                <h3 class="text-3xl font-bold">{"5"}</h3>
                            </div>
                            <div class={classes!(bg_primary, "p-3", "rounded-full")}>
                                <i class="fas fa-code-branch text-xl"></i>
                            </div>
                        </div>
                    </div>
                </div>


                <div class={classes!("mt-8", bg_secondary, "rounded-lg", "shadow", "overflow-hidden")}>
                    <div class={classes!("p-4", "border-b", border_color, "flex", "justify-between", "items-center")}>
                        <h3 class={classes!("text-xl", "font-bold", text_primary)}>{"Blog Posts"}</h3>
                        <button class={classes!(bg_primary, text_secondary, "px-4", "py-2", "rounded-md", "hover:opacity-90")}>
                            <i class="fas fa-plus mr-2"></i>{"New Post"}
                        </button>
                    </div>
                    <div class="overflow-x-auto">
                        <table class="min-w-full">
                            <thead class={classes!(bg_primary, "border-b", border_color)}>
                                <tr>
                                    <th class={classes!("px-6", "py-3", "text-left", "text-xs", "font-medium", text_secondary, "uppercase", "tracking-wider")}>{"Title"}</th>
                                    <th class={classes!("px-6", "py-3", "text-left", "text-xs", "font-medium", text_secondary, "uppercase", "tracking-wider")}>{"Date"}</th>
                                    <th class={classes!("px-6", "py-3", "text-left", "text-xs", "font-medium", text_secondary, "uppercase", "tracking-wider")}>{"Tags"}</th>
                                    <th class={classes!("px-6", "py-3", "text-left", "text-xs", "font-medium", text_secondary, "uppercase", "tracking-wider")}>{"Actions"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr class={classes!(bg_secondary, "border-b", border_color, "last:border-0")}>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>{"Getting Started with Rust & Yew"}</td>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>{"Apr 12, 2023"}</td>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>
                                        <span class="px-2 py-1 text-xs rounded bg-red-600 text-white mr-1">{"rust"}</span>
                                        <span class="px-2 py-1 text-xs rounded bg-blue-600 text-white">{"yew"}</span>
                                    </td>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>
                                        <button class={classes!(text_primary, "hover:opacity-70", "mr-3")}><i class="fas fa-edit"></i></button>
                                        <button class="text-red-500 hover:opacity-70"><i class="fas fa-trash"></i></button>
                                    </td>
                                </tr>
                                <tr class={classes!(bg_secondary, "border-b", border_color, "last:border-0")}>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>{"Web Development with Rust"}</td>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>{"Mar 28, 2023"}</td>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>
                                        <span class="px-2 py-1 text-xs rounded bg-green-600 text-white mr-1">{"web"}</span>
                                        <span class="px-2 py-1 text-xs rounded bg-purple-600 text-white">{"tutorial"}</span>
                                    </td>
                                    <td class={classes!("px-6", "py-4", "whitespace-nowrap", text_primary)}>
                                        <button class={classes!(text_primary, "hover:opacity-70", "mr-3")}><i class="fas fa-edit"></i></button>
                                        <button class="text-red-500 hover:opacity-70"><i class="fas fa-trash"></i></button>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </main>
        </div>
    }
}
