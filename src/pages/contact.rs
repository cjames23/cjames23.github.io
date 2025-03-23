// src/pages/contact.rs
use crate::app::ThemeContext;
use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
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
    let form_data = use_state(|| FormData::default());
    let submitting = use_state(|| false);
    let status_message = use_state(|| None::<(String, bool)>);

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
                        status_message.set(Some(("Failed to process form data".to_string(), false)));
                        submitting.set(false);
                        return;
                    }
                };

                // Create request with error handling
                let request = match Request::post("/api/contact")
                    .header("Content-Type", "application/json")
                    .body(json_body) {
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
                    },
                    Err(_) => {
                        status_message.set(Some(("Failed to send message. Please try again.".to_string(), false)));
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
        <div class="page-container p-8">
            <h1 class="text-3xl font-bold mb-6">{"Contact Me"}</h1>

            <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-6">
                <form {onsubmit} class="space-y-4">
                    {
                        if let Some((message, is_success)) = &*status_message {
                            let class = if *is_success {
                                "p-4 mb-4 text-sm text-green-700 bg-green-100 rounded-lg dark:bg-green-200 dark:text-green-800"
                            } else {
                                "p-4 mb-4 text-sm text-red-700 bg-red-100 rounded-lg dark:bg-red-200 dark:text-red-800"
                            };
                            html! { <div class={class}>{message}</div> }
                        } else {
                            html! {}
                        }
                    }

                    <div>
                        <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">{"Name"}</label>
                        <input
                            type="text"
                            id="name"
                            name="name"
                            value={form_data.name.clone()}
                            onchange={&handle_input}
                            required=true
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                        />
                    </div>

                    <div>
                        <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300">{"Email"}</label>
                        <input
                            type="email"
                            id="email"
                            name="email"
                            value={form_data.email.clone()}
                            onchange={&handle_input}
                            required=true
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                        />
                    </div>

                    <div>
                        <label for="message" class="block text-sm font-medium text-gray-700 dark:text-gray-300">{"Message"}</label>
                        <textarea
                            id="message"
                            name="message"
                            value={form_data.message.clone()}
                            onchange={&handle_input}
                            required=true
                            rows="4"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                        ></textarea>
                    </div>

                    <button
                        type="submit"
                        disabled={*submitting}
                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
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
            </div>
        </div>
    }
}