use crate::app::ThemeContext;
use crate::data::blog::BlogDb;
use crate::route::Route;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub slug: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let blog_db = use_memo((), |_| BlogDb::new());
    let navigator = use_navigator().unwrap();

    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
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
    let border_color = if dark_mode {
        "border-[#739072]"
    } else {
        "border-[#4F6F52]"
    };

    let post = blog_db.get_post_by_slug(&props.slug);

    let go_back = {
        let nav = navigator.clone();
        Callback::from(move |_| nav.push(&Route::Blog))
    };

    match post {
        Some(post) => {
            let formatted_date = post.created_at.format("%B %d, %Y").to_string();
            let paragraphs: Vec<String> = post
                .content
                .replace('\u{2028}', "\n")
                .split("\n\n")
                .map(str::trim)
                .filter(|paragraph| !paragraph.is_empty())
                .map(ToString::to_string)
                .collect();
            let reading_time = (post.content.split_whitespace().count() / 220).max(1);

            html!(
                <div class={classes!("page-container", "p-4", "md:p-8", text_primary)}>
                    <div class="max-w-3xl mx-auto">
                        <div class={classes!("mb-4", text_primary)}>
                            <Breadcrumb>
                                <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                                <BreadcrumbItem href="/blog">{"Blog"}</BreadcrumbItem>
                                <BreadcrumbItem>{&post.title}</BreadcrumbItem>
                            </Breadcrumb>
                        </div>

                        <button
                            class={classes!("mb-6", "inline-flex", "items-center", "text-sm", text_primary, "hover:underline")}
                            onclick={go_back.clone()}
                        >
                            <i class="fas fa-arrow-left mr-2"></i>
                            {"Back to Blog"}
                        </button>

                        <Card
                            size={CardSize::Large}
                            class={classes!(card_bg, card_text, "shadow-md", "rounded-lg", "border", border_color)}
                        >
                            <CardBody>
                                <article class="max-w-none">
                                    <header class={classes!("pb-6", "mb-6", "border-b", border_color)}>
                                        <h1 class="text-3xl md:text-4xl font-bold leading-tight">{&post.title}</h1>
                                        <div class={classes!("mt-4", "text-sm", "flex", "flex-wrap", "items-center", "gap-x-4", "gap-y-2", text_primary)}>
                                            <span class="inline-flex items-center">
                                                <i class="fas fa-user mr-2"></i>
                                                {&post.author}
                                            </span>
                                            <span class="inline-flex items-center">
                                                <i class="far fa-calendar-alt mr-2"></i>
                                                {formatted_date}
                                            </span>
                                            <span class="inline-flex items-center">
                                                <i class="far fa-clock mr-2"></i>
                                                {format!("{reading_time} min read")}
                                            </span>
                                        </div>
                                        <div class="mt-4 flex flex-wrap gap-2">
                                            {
                                                post.tags.iter().map(|tag| {
                                                    html! {
                                                        <span class={classes!(bg_primary, text_primary, "px-3", "py-1", "rounded-full", "text-xs", "uppercase", "tracking-wide")}>
                                                            {tag}
                                                        </span>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </header>

                                    <section class="space-y-5 text-base leading-8 md:text-lg">
                                        {
                                            paragraphs.iter().map(|paragraph| {
                                                html! { <p>{paragraph}</p> }
                                            }).collect::<Html>()
                                        }
                                    </section>
                                </article>
                            </CardBody>
                        </Card>

                        <div class={classes!("mt-6", "pt-4", "border-t", border_color)}>
                            <button
                                class={classes!("inline-flex", "items-center", "text-sm", text_primary, "hover:underline")}
                                onclick={go_back}
                            >
                                <i class="fas fa-arrow-left mr-2"></i>
                                {"Back to Blog"}
                            </button>
                        </div>
                    </div>
                </div>
            )
        }
        None => html!(
            <div class={classes!("page-container", "p-8", text_primary)}>
                <div class="max-w-2xl mx-auto">
                    <Content>
                        <h1 class="text-2xl font-bold mb-4">{"Blog Post Not Found"}</h1>
                        <p class="mb-4">{"The blog post you're looking for doesn't exist."}</p>
                        <Button
                            onclick={go_back}
                            class={classes!(bg_secondary, text_primary, "hover:opacity-90")}
                        >
                            {"Back to Blog"}
                        </Button>
                    </Content>
                </div>
            </div>
        ),
    }
}
