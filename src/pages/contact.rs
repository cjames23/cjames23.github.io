// src/pages/contact.rs
use crate::app::ThemeContext;
use gloo::net::http::Request;
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct FormData {
    name: String,
    email: String,
    message: String,
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;
    let form_data = use_state(|| FormData::default());
    let submitting = use_state(|| false);
    let status_message = use_state(|| None::<(String, bool)>);

    // Define color palette based on theme
    let bg_primary = if dark_mode {
        "bg-[#3A4D39]"
    } else {
        "bg-[#ECE3CE]"
    };
    let text_primary = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#3A4D39]"
    };
    let card_bg = if dark_mode {
        "bg-[#4F6F52]"
    } else {
        "bg-[#739072]"
    };
    let card_text = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#ECE3CE]"
    };
    let input_bg = if dark_mode {
        "bg-[#3A4D39]"
    } else {
        "bg-[#ECE3CE]"
    };
    let input_border = if dark_mode {
        "border-[#ECE3CE]"
    } else {
        "border-[#3A4D39]"
    };
    let input_text = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#3A4D39]"
    };
    let button_bg = if dark_mode {
        "bg-[#3A4D39]"
    } else {
        "bg-[#3A4D39]"
    };
    let button_text = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#ECE3CE]"
    };
    let button_hover = if dark_mode {
        "hover:bg-opacity-80"
    } else {
        "hover:bg-opacity-80"
    };

    let onsubmit = {
        let form_data = form_data.clone();
        let submitting = submitting.clone();
        let status_message = status_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if *submitting {
                return;
            }

            let form_data = form_data.clone();
            let submitting = submitting.clone();
            let status_message = status_message.clone();

            submitting.set(true);
            status_message.set(None);

            // Email validation
            if !form_data.email.contains('@') {
                status_message.set(Some(("Please enter a valid email".to_string(), false)));
                submitting.set(false);
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                // Serialize form data with proper error handling
                let json_body = match serde_json::to_string(&*form_data) {
                    Ok(body) => body,
                    Err(_) => {
                        status_message
                            .set(Some(("Failed to process form data".to_string(), false)));
                        submitting.set(false);
                        return;
                    }
                };

                // Create request with error handling
                let request = match Request::post("/api/contact")
                    .header("Content-Type", "application/json")
                    .body(json_body)
                {
                    Ok(req) => req,
                    Err(_) => {
                        status_message.set(Some(("Failed to create request".to_string(), false)));
                        submitting.set(false);
                        return;
                    }
                };

                // Send request
                match request.send().await {
                    Ok(_) => {
                        status_message.set(Some(("Message sent successfully!".to_string(), true)));
                        form_data.set(FormData::default());
                    }
                    Err(_) => {
                        status_message.set(Some((
                            "Failed to send message. Please try again.".to_string(),
                            false,
                        )));
                    }
                }

                submitting.set(false);
            });
        })
    };

    let handle_input = {
        let form_data = form_data.clone();

        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            let field_name = input.name();
            let value = input.value();

            let mut updated = (*form_data).clone();
            match field_name.as_str() {
                "name" => updated.name = value,
                "email" => updated.email = value,
                "message" => updated.message = value,
                _ => {}
            }

            form_data.set(updated);
        })
    };

    html! {
        <div class={classes!("page-container", "p-8", text_primary)}>
            <Breadcrumb>
                <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                <BreadcrumbItem href="/contact">{"Contact"}</BreadcrumbItem>
            </Breadcrumb>

            <h1 class="text-3xl font-bold my-6">{"Contact Me"}</h1>

            <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                <CardTitle>{"Send a Message"}</CardTitle>
                <CardBody>
                    <form {onsubmit} class="space-y-4">
                        {
                            if let Some((message, is_success)) = &*status_message {
                                let status_bg = if *is_success {
                                    if dark_mode { "bg-green-700" } else { "bg-green-600" }
                                } else {
                                    if dark_mode { "bg-red-700" } else { "bg-red-600" }
                                };

                                html! {
                                    <div class={classes!("p-4", "rounded-lg", status_bg, "text-white")}>
                                        {message}
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }

                        <div>
                            <label for="name" class="block text-sm font-medium mb-1">{"Name"}</label>
                            <input
                                type="text"
                                id="name"
                                name="name"
                                value={form_data.name.clone()}
                                onchange={&handle_input}
                                required=true
                                class={classes!(
                                    "mt-1",
                                    "block",
                                    "w-full",
                                    "rounded-md",
                                    "border",
                                    input_border,
                                    input_bg,
                                    input_text,
                                    "shadow-sm",
                                    "focus:ring-2",
                                    "focus:ring-opacity-50",
                                    "p-2"
                                )}
                            />
                        </div>

                        <div>
                            <label for="email" class="block text-sm font-medium mb-1">{"Email"}</label>
                            <input
                                type="email"
                                id="email"
                                name="email"
                                value={form_data.email.clone()}
                                onchange={&handle_input}
                                required=true
                                class={classes!(
                                    "mt-1",
                                    "block",
                                    "w-full",
                                    "rounded-md",
                                    "border",
                                    input_border,
                                    input_bg,
                                    input_text,
                                    "shadow-sm",
                                    "focus:ring-2",
                                    "focus:ring-opacity-50",
                                    "p-2"
                                )}
                            />
                        </div>

                        <div>
                            <label for="message" class="block text-sm font-medium mb-1">{"Message"}</label>
                            <textarea
                                id="message"
                                name="message"
                                value={form_data.message.clone()}
                                onchange={&handle_input}
                                required=true
                                rows="4"
                                class={classes!(
                                    "mt-1",
                                    "block",
                                    "w-full",
                                    "rounded-md",
                                    "border",
                                    input_border,
                                    input_bg,
                                    input_text,
                                    "shadow-sm",
                                    "focus:ring-2",
                                    "focus:ring-opacity-50",
                                    "p-2",
                                    "resize-y"
                                )}
                            ></textarea>
                        </div>

                        <button
                            type="submit"
                            disabled={*submitting}
                            class={classes!(
                                "inline-flex",
                                "items-center",
                                "px-4",
                                "py-2",
                                "border",
                                "border-transparent",
                                "text-sm",
                                "font-medium",
                                "rounded-md",
                                "shadow-sm",
                                button_bg,
                                button_text,
                                button_hover,
                                "focus:outline-none",
                                "focus:ring-2",
                                "disabled:opacity-50",
                                "transition-colors"
                            )}
                        >
                            {
                                if *submitting {
                                    html! { <span>{"Sending..."}</span> }
                                } else {
                                    html! { <span>{"Send Message"}</span> }
                                }
                            }
                        </button>
                    </form>
                </CardBody>
            </Card>
        </div>
    }
}
