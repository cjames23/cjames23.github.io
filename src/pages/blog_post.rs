use crate::app::ThemeContext;
use crate::data::blog::BlogDb;
use crate::route::Route;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub id: String,
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

    let post = blog_db.get_post(&props.id);

    let go_back = {
        let nav = navigator.clone();
        Callback::from(move |_| nav.push(&Route::Blog))
    };

    match post {
        Some(post) => {
            let formatted_date = post.created_at.format("%B %d %Y %H:%M%P").to_string();

            html!(
                <div class={classes!("page-container", "p-8", text_primary)}>
                    <Breadcrumb>
                        <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                        <BreadcrumbItem href="/blog">{"Blog"}</BreadcrumbItem>
                        <BreadcrumbItem>{&post.title}</BreadcrumbItem>
                    </Breadcrumb>

                    <Button
                        variant={ButtonVariant::Secondary}
                        onclick={go_back}
                        class={classes!("mt-4", "mb-4")}
                    >
                        <i class="fas fa-arrow-left mr-2"></i>{"Back to Blog"}
                    </Button>

                    <Card
                        size={CardSize::Large}
                        class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}
                    >
                        <CardTitle>
                            <h1 class="text-2xl font-bold">{&post.title}</h1>
                        </CardTitle>
                        <CardBody>
                            <div class="mb-4 flex items-center">
                                <div class={classes!("h-8", "w-8", "rounded-full", bg_primary, "flex", "items-center", "justify-center", "mr-2")}>
                                    <i class="fas fa-user"></i>
                                </div>
                                <span>{&post.author}</span>
                                <span class="mx-2">{"·"}</span>
                                <span>
                                    <i class="far fa-calendar-alt mr-1"></i>
                                    {formatted_date}
                                </span>
                            </div>

                            <div class="tag-icons mb-4 flex flex-wrap gap-2">
                                {
                                    post.tags.iter().map(|tag| {
                                        html! {
                                            <span class={classes!(bg_primary, text_primary, "px-3", "py-1", "rounded", "text-sm")}>
                                                {tag}
                                            </span>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>

                            <div class="post-content">
                                {&post.content}
                            </div>
                        </CardBody>
                    </Card>
                </div>
            )
        }
        None => html!(
            <div class={classes!("page-container", "p-8", text_primary)}>
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
        ),
    }
}
