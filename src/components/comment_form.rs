use crate::app::ThemeContext;
use crate::data::comment::Comment;
use chrono::Utc;
use gloo::utils::document;
use patternfly_yew::prelude::OnOptionSelectArgs;
use uuid::Uuid;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CommentFormProps {
    pub post_id: String,
    pub on_submit: Callback<Comment>,
    pub on_cancel: Callback<()>,
}

#[function_component(CommentForm)]
pub fn comment_form(props: &CommentFormProps) -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;
    let author = use_state(|| String::new());
    let content = use_state(|| String::new());

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
    let button_cancel_bg = if dark_mode {
        "bg-red-700"
    } else {
        "bg-red-600"
    };

    let on_submit = {
        let author = author.clone();
        let content = content.clone();
        let post_id = props.post_id.clone();
        let on_submit = props.on_submit.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !author.is_empty() && !content.is_empty() {
                let comment = Comment {
                    id: Uuid::new_v4().to_string(),
                    post_id: post_id.clone(),
                    author: (*author).clone(),
                    content: (*content).clone(),
                    created_at: Utc::now(),
                };
                on_submit.emit(comment);
            }
        })
    };

    let handle_author_change = {
        let author = author.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_into::<HtmlInputElement>().unwrap_throw();
            author.set(input.value())
        })
    };
    let handle_content_change = {
        let content = content.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_into::<HtmlInputElement>().unwrap_throw();
            content.set(input.value());
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };
    html! {
         <form onsubmit={on_submit} class="space-y-4">
            <div>
                <label for="comment-name" class="block text-sm font-medium mb-1">{"Your Name"}</label>
                <input
                    type="text"
                    id="comment-name"
                    value={(*author).clone()}
                    onchange={handle_author_change}
                    required=true
                    class={classes!(
                        "mt-1", "block", "w-full", "rounded-md", "border",
                        input_border, input_bg, input_text,
                        "shadow-sm", "focus:ring-2", "focus:ring-opacity-50", "p-2"
                    )}
                />
            </div>

            <div>
                <label for="comment-content" class="block text-sm font-medium mb-1">{"Comment"}</label>
                <textarea
                    id="comment-content"
                    value={(*content).clone()}
                    onchange={handle_content_change}
                    required=true
                    rows="4"
                    class={classes!(
                        "mt-1", "block", "w-full", "rounded-md", "border",
                        input_border, input_bg, input_text,
                        "shadow-sm", "focus:ring-2", "focus:ring-opacity-50", "p-2", "resize-y"
                    )}
                ></textarea>
            </div>

            <div class="flex justify-end space-x-3">
                <button
                    type="button"
                    onclick={on_cancel}
                    class={classes!(
                        "px-4", "py-2", "border", "border-transparent", "text-sm", "font-medium",
                        "rounded-md", "shadow-sm", button_cancel_bg, "text-white", button_hover,
                        "focus:outline-none", "focus:ring-2", "transition-colors"
                    )}
                >
                    {"Cancel"}
                </button>

                <button
                    type="submit"
                    class={classes!(
                        "px-4", "py-2", "border", "border-transparent", "text-sm", "font-medium",
                        "rounded-md", "shadow-sm", button_bg, button_text, button_hover,
                        "focus:outline-none", "focus:ring-2", "transition-colors"
                    )}
                >
                    {"Post Comment"}
                </button>
            </div>
        </form>
    }
}
