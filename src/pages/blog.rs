// src/pages/blog.rs
use crate::app::ThemeContext;
use crate::components::comment_form::CommentForm;
use crate::components::modal::ModalCom;
use crate::data::blog::BlogDb;
use crate::data::comment::{Comment, CommentDb};
use crate::route::Route;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let blog_db = use_memo((), |_| BlogDb::new());
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

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

    let comment_db = use_state(|| CommentDb::new());
    let show_comment_modal = use_state(|| false);
    let active_post_id = use_state(|| String::new());

    let handle_comment_click = {
        let show_comment_modal = show_comment_modal.clone();
        let active_post_id = active_post_id.clone();
        Callback::from(move |post_id: String| {
            show_comment_modal.set(true);
            active_post_id.set(post_id);
        })
    };

    let handle_comment_submit = {
        let comment_db = comment_db.clone();
        let show_comment_modal = show_comment_modal.clone();

        Callback::from(move |comment: Comment| {
            let mut db = (*comment_db).clone();
            db.add_comment(comment);
            comment_db.set(db);
            show_comment_modal.set(false);
        })
    };

    let handle_modal_close = {
        let show_comment_modal = show_comment_modal.clone();

        Callback::from(move |_| {
            // Changed to accept () instead of MouseEvent
            show_comment_modal.set(false);
        })
    };

    html! {
            <div class={classes!("min-h-screen", bg_primary)}>
                <Split>
                    <SplitItem>
                    </SplitItem>
                    <SplitItem fill=true>
                        <div class="w-full container mx-auto px-4 py-6">
                            // Wrap the entire breadcrumb section in a div with our custom styling
                            <div class={classes!("mb-4", text_primary)}>
                                <Breadcrumb>
                                    <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                                    <BreadcrumbItem href="/blog">{"Blog"}</BreadcrumbItem>
                                </Breadcrumb>
                            </div>

                            <div class={classes!("mb-6", "p-4", "rounded-lg", bg_secondary, "shadow")}>
                                <div class="flex flex-col lg:flex-row justify-between items-start lg:items-center space-y-4 lg:space-y-0">
                                    <h1 class={classes!("text-2xl", "font-bold", text_primary)}>{"Blog Posts"}</h1>

                                    <div class="flex flex-col sm:flex-row items-start sm:items-center w-full lg:w-auto space-y-3 sm:space-y-0 sm:space-x-4">
                                        <div class="relative flex-grow sm:w-64">
                                            <input
                                                type="text"
                                                placeholder="Search posts..."
                                                class={classes!(
                                                    "w-full", "px-4", "py-2", "pl-10", "rounded-md",
                                                    bg_primary, text_primary, "border", border_color,
                                                    "focus:outline-none", "focus:ring-2", "focus:ring-opacity-50",
                                                    "focus:ring-[#ECE3CE]", "placeholder-opacity-75"
                                                )}
                                            />
                                            <div class="absolute inset-y-0 left-0 flex items-center pl-3">
                                                <i class={classes!("fas", "fa-search", text_primary, "opacity-70")}></i>
                                            </div>
                                        </div>

                                        <select
                                            class={classes!(
                                                "px-4", "py-2", "rounded-md", "appearance-none",
                                                bg_primary, text_primary, "border", border_color,
                                                "focus:outline-none", "focus:ring-2", "focus:ring-opacity-50",
                                                "focus:ring-[#ECE3CE]", "pr-8"
                                            )}>
                                            <option value="">{"All Tags"}</option>
                                            <option value="rust">{"Rust"}</option>
                                            <option value="yew">{"Yew"}</option>
                                            <option value="web">{"Web"}</option>
                                            <option value="tutorial">{"Tutorial"}</option>
                                        </select>

                                        <select
                                            class={classes!(
                                                "px-4", "py-2", "rounded-md", "appearance-none",
                                                bg_primary, text_primary, "border", border_color,
                                                "focus:outline-none", "focus:ring-2", "focus:ring-opacity-50",
                                                "focus:ring-[#ECE3CE]", "pr-8"
                                            )}>
                                            <option value="newest">{"Newest First"}</option>
                                            <option value="oldest">{"Oldest First"}</option>
                                        </select>

                                        <button
                                            class={classes!(
                                                "px-4", "py-2", "rounded-md", "flex", "items-center",
                                                bg_primary, text_primary, "hover:opacity-90", "transition-opacity"
                                            )}
                                        >
                                            <i class="fas fa-th-large mr-2"></i>
                                            {"View"}
                                        </button>
                                    </div>
                                </div>
                            </div>

                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                {
                                    for blog_db.get_all_posts().into_iter().map(|post| {
                                        let post_id = post.id.clone();
                                        let formatted_date = post.created_at.format("%B %d, %Y").to_string();
                                        let comment_count = comment_db.get_comments(&post_id).len();

                                        html! {
                                            <div class={classes!(
                                                "rounded-lg", "overflow-hidden", "shadow-lg", "flex", "flex-col",
                                                bg_secondary, "border", border_color,
                                                "transition-transform", "duration-300", "hover:scale-[1.02]"
                                            )}>
                                                <div class={classes!(bg_primary, "h-40", "flex", "items-center", "justify-center")}>
                                                    <i class={classes!("fas", "fa-newspaper", "text-5xl", text_secondary)}></i>
                                                </div>

                                                <div class="p-6 flex-grow">
                                                    <Link<Route>
                                                        to={Route::BlogPost { id: post_id.clone() }}
                                                        classes={classes!("text-xl", "font-semibold", "block", "mb-2", text_primary, "hover:underline")}
                                                    >
                                                        {&post.title}
                                                    </Link<Route>>

                                                    <div class={classes!("mb-4", text_primary, "opacity-80")}>
                                                        {post.content.chars().take(120).collect::<String>()}
                                                        {"..."}
                                                    </div>

                                                    <div class="flex flex-wrap gap-2 mb-4">
                                                        {
                                                            post.tags.iter().map(|tag| {
                                                                let tag_bg = match tag.as_str() {
                                                                    "rust" => "bg-red-600",
                                                                    "yew" => "bg-blue-600",
                                                                    "web" => "bg-green-600",
                                                                    "tutorial" => "bg-purple-600",
                                                                    _ => bg_primary
                                                                };
                                                                html! {
                                                                    <span class={classes!(tag_bg, "text-white", "px-2", "py-1", "rounded", "text-xs")}>
                                                                        {tag}
                                                                    </span>
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </div>
                                                </div>

                                                <div class={classes!(
                                                    "px-6", "py-4", "border-t", border_color,
                                                    "flex", "justify-between", "items-center"
                                                )}>
                                                    <div class={classes!("flex", "items-center", text_primary)}>
                                                        <div class={classes!("h-8", "w-8", "rounded-full", bg_primary, "flex", "items-center", "justify-center", "mr-2")}>
                                                            <i class="fas fa-user"></i>
                                                        </div>
                                                        <span>{&post.author}</span>
                                                    </div>

                                                    <div class={classes!(text_primary, "opacity-80", "text-sm")}>
                                                        <i class="far fa-calendar-alt mr-1"></i>
                                                        {formatted_date}
                                                    </div>
                                                </div>

                                                <div class={classes!(
                                                    "px-6", "py-3", "bg-opacity-50", bg_primary,
                                                    "flex", "justify-between", "items-center"
                                                )}>
                                                    <div class="flex space-x-4">
                                                        <button class={classes!(text_primary, "hover:opacity-80")}>
                                                            <i class="far fa-heart mr-1"></i>
                                                            {"Like"}
                                                        </button>
                                                        <button class={classes!(text_primary, "hover:opacity-80")}
                                                        onclick={
                                                            let post_id = post.id.clone();
                                                            let handle_comment_click = handle_comment_click.clone();
                                                            Callback::from(move |_| handle_comment_click.emit(post_id.clone()))
                                                        }>
                                                            <i class="far fa-comment mr-1"></i>
                                                            {format!("Comment ({})", comment_count)}
                                                        </button>
                                                    </div>
                                                    <Link<Route>
                                                        to={Route::BlogPost { id: post_id }}
                                                        classes={classes!("px-4", "py-1", "rounded", bg_secondary, text_primary, "hover:opacity-90")}
                                                    >
                                                        {"Read More"}
                                                    </Link<Route>>
                                                </div>
                                            </div>
                                        }
                                    })
                                }
                            </div>

                            <div class={classes!("mt-8", "flex", "justify-center")}>
                                <nav>
                                    <ul class="flex space-x-2">
                                        <li>
                                            <button class={classes!(
                                                "px-3", "py-1", "rounded", bg_secondary, text_primary, "hover:opacity-90",
                                                "flex", "items-center"
                                            )}>
                                                <i class="fas fa-chevron-left mr-1"></i>
                                                {"Previous"}
                                            </button>
                                        </li>
                                        <li>
                                            <button class={classes!(bg_primary, text_secondary, "px-3", "py-1", "rounded", "font-bold")}>{"1"}</button>
                                        </li>
                                        <li>
                                            <button class={classes!(bg_secondary, text_primary, "px-3", "py-1", "rounded")}>{"2"}</button>
                                        </li>
                                        <li>
                                            <button class={classes!(bg_secondary, text_primary, "px-3", "py-1", "rounded")}>{"3"}</button>
                                        </li>
                                        <li>
                                            <button class={classes!(
                                                "px-3", "py-1", "rounded", bg_secondary, text_primary, "hover:opacity-90",
                                                "flex", "items-center"
                                            )}>
                                                {"Next"}
                                                <i class="fas fa-chevron-right ml-1"></i>
                                            </button>
                                        </li>
                                    </ul>
                                </nav>
                            </div>
                        </div>
                    </SplitItem>
                </Split>
            <ModalCom
        title="Add a Comment"
        is_open={*show_comment_modal}
        on_close={handle_modal_close.clone()}
    >
        <CommentForm
            post_id={(*active_post_id).clone()}
            on_submit={handle_comment_submit}
            on_cancel={handle_modal_close.clone()} // This will now match the expected type
        />
    </ModalCom>
            </div>
        }
}
