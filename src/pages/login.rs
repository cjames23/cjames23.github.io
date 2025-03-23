// src/pages/login.rs
use crate::app::ThemeContext;
use crate::route::Route;
use gloo::storage::{LocalStorage, Storage};
use patternfly_yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error_message = use_state(|| String::new());
    let loading = use_state(|| false);

    let navigator = use_navigator().unwrap();

    // Define color palette as variables
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

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error_message = error_message.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username_val = (*username).clone();
            let password_val = (*password).clone();
            let error_message = error_message.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();

            // Simple validation
            if username_val.is_empty() || password_val.is_empty() {
                error_message.set("Username and password are required".to_string());
                return;
            }

            loading.set(true);

            // In a real app, this would be an API call
            spawn_local(async move {
                // Simulate API delay
                gloo::timers::future::TimeoutFuture::new(1000).await;

                // Simple hardcoded auth for demo purposes
                // In production, this should be a secure API call
                if username_val == "admin" && password_val == "password123" {
                    // Store auth token
                    let _ = LocalStorage::set("auth_token", "demo_token_12345");
                    loading.set(false);
                    navigator.push(&Route::Admin);
                } else {
                    error_message.set("Invalid username or password".to_string());
                    loading.set(false);
                }
            });
        })
    };

    let go_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    html! {
        <div class={classes!("min-h-screen", "flex", "flex-col", "items-center", "justify-center", "px-4", bg_primary)}>
            <div class={classes!("w-full", "max-w-md", bg_secondary, "rounded-lg", "shadow-xl", "overflow-hidden")}>
                <div class={classes!("p-6")}>
                    <div class="text-center mb-6">
                        <h2 class={classes!("text-2xl", "font-bold", text_primary)}>{"Admin Login"}</h2>
                        <p class={classes!("mt-2", text_primary, "opacity-80")}>{"Sign in to access the admin dashboard"}</p>
                    </div>

                    <form onsubmit={on_submit}>
                        <div class="space-y-4">
                            <div>
                                <label for="username" class={classes!("block", "text-sm", "font-medium", "mb-1", text_primary)}>
                                    {"Username"}
                                </label>
                                <input
                                    type="text"
                                    id="username"
                                    value={(*username).clone()}
                                    onchange={on_username_change}
                                    class={classes!(
                                        "w-full", "px-3", "py-2", "rounded-md", bg_primary, text_secondary,
                                        "border", border_color, "focus:outline-none", "focus:ring-2"
                                    )}
                                    placeholder="Enter your username"
                                />
                            </div>

                            <div>
                                <label for="password" class={classes!("block", "text-sm", "font-medium", "mb-1", text_primary)}>
                                    {"Password"}
                                </label>
                                <input
                                    type="password"
                                    id="password"
                                    value={(*password).clone()}
                                    onchange={on_password_change}
                                    class={classes!(
                                        "w-full", "px-3", "py-2", "rounded-md", bg_primary, text_secondary,
                                        "border", border_color, "focus:outline-none", "focus:ring-2"
                                    )}
                                    placeholder="Enter your password"
                                />
                            </div>

                            {
                                if !(*error_message).is_empty() {
                                    html! {
                                        <div class="p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                                            {&*error_message}
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }

                            <div class="flex items-center justify-between pt-2">
                                <button
                                    type="button"
                                    onclick={go_back}
                                    class={classes!(
                                        "px-4", "py-2", "rounded-md",
                                        "border", border_color,
                                        text_primary,
                                        "hover:opacity-90"
                                    )}>
                                    {"Back"}
                                </button>

                                <button
                                    type="submit"
                                    disabled={*loading}
                                    class={classes!(
                                        "px-4", "py-2", "rounded-md",
                                        bg_primary, text_secondary,
                                        "hover:opacity-90",
                                        "flex", "items-center", "justify-center",
                                        "min-w-[100px]"
                                    )}>
                                    {
                                        if *loading {
                                            html! { <span class="animate-spin mr-2">{"⌛"}</span> }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {"Login"}
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
