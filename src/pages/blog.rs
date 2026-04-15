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

                            <div class={classes!("mb-8", "p-5", "rounded-lg", bg_secondary, "shadow")}>
                                <h1 class={classes!("text-3xl", "font-bold", "mb-2", text_primary)}>{"Blog"}</h1>
                                <p class={classes!(text_primary, "opacity-90", "max-w-3xl")}>
                                    {"Notes on software, philosophy, and the strange places where engineering meets theory."}
                                </p>
                            </div>

                            <div class="space-y-6">
                                {
                                    for blog_db.get_all_posts().into_iter().map(|post| {
                                        let post_id = post.id.clone();
                                        let post_slug = post.slug();
                                        let formatted_date = post.created_at.format("%B %d, %Y").to_string();
                                        let comment_count = comment_db.get_comments(&post_id).len();
                                        let excerpt = post
                                            .content
                                            .replace('\u{2028}', " ")
                                            .replace('\n', " ")
                                            .chars()
                                            .take(280)
                                            .collect::<String>();

                                        html! {
                                            <article class={classes!(
                                                "rounded-lg", "shadow-lg", "p-6", "md:p-8",
                                                bg_secondary, "border", border_color
                                            )}>
                                                <div class={classes!("text-sm", "mb-3", text_primary, "opacity-80", "flex", "flex-wrap", "items-center", "gap-3")}>
                                                    <span class="inline-flex items-center">
                                                        <i class="far fa-calendar-alt mr-2"></i>
                                                        {formatted_date}
                                                    </span>
                                                    <span class="inline-flex items-center">
                                                        <i class="fas fa-user mr-2"></i>
                                                        {&post.author}
                                                    </span>
                                                </div>

                                                <div class="mb-4">
                                                    <Link<Route>
                                                        to={Route::BlogPost { slug: post_slug.clone() }}
                                                        classes={classes!("text-2xl", "md:text-3xl", "font-bold", "leading-tight", "block", "mb-3", text_primary, "hover:underline")}
                                                    >
                                                        {&post.title}
                                                    </Link<Route>>
                                                    <p class={classes!("mb-4", text_primary, "opacity-90", "leading-7")}>
                                                        {excerpt}
                                                        {"..."}
                                                    </p>

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
                                                    "pt-4", "border-t", border_color,
                                                    "flex", "flex-wrap", "justify-between", "gap-3", "items-center"
                                                )}>
                                                    <button class={classes!(text_primary, "hover:opacity-80")}
                                                        onclick={
                                                            let post_id = post.id.clone();
                                                            let handle_comment_click = handle_comment_click.clone();
                                                            Callback::from(move |_| handle_comment_click.emit(post_id.clone()))
                                                        }>
                                                            <i class="far fa-comment mr-1"></i>
                                                            {format!("Comment ({})", comment_count)}
                                                    </button>
                                                    <Link<Route>
                                                        to={Route::BlogPost { slug: post_slug }}
                                                        classes={classes!("px-4", "py-2", "rounded", bg_primary, text_primary, "font-medium", "hover:opacity-90")}
                                                    >
                                                        {"Read Article"}
                                                    </Link<Route>>
                                                </div>
                                            </article>
                                        }
                                    })
                                }
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
