use crate::components::nav::Nav;
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

    let post = blog_db.get_post(&props.id);

    let go_back = {
        let nav = navigator.clone();
        Callback::from(move |_| nav.push(&Route::Blog))
    };

    match post {
        Some(post) => {
            let formatted_date = post.created_at.format("%B %d %Y %H:%M%P").to_string();

            html!(
                <>
                <Split>
                    <SplitItem>
                        <Nav/>
                    </SplitItem>
                    <SplitItem>
                        <Breadcrumb>
                            <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                            <BreadcrumbItem href="/blog">{"Blog"}</BreadcrumbItem>
                            <BreadcrumbItem>{&post.title}</BreadcrumbItem>
                        </Breadcrumb>
                        <Content>
                            <Button variant={ButtonVariant::Secondary} onclick={go_back}>
                                <i class="fas fa-arrow-left"></i>{" Back to Blog"}
                            </Button>

                            <Card size={CardSize::Large} class="mt-3">
                                <CardTitle>
                                    <h1>{&post.title}</h1>
                                </CardTitle>
                                <CardBody>
                                    <div class="text-muted mb-4">
                                        {format!("By {} on {}", post.author, formatted_date)}
                                    </div>

                                    <div class="tag-icons mb-4">
                                        {
                                            post.tags.iter().map(|tag| {
                                                let color = match tag.as_str() {
                                                    "rust" => Color::Red,
                                                    "yew" => Color::Blue,
                                                    "web" => Color::Green,
                                                    "tutorial" => Color::Purple,
                                                    _ => Color::Grey
                                                };
                                                html!(<Label {color} label={tag.clone()} />)
                                            }).collect::<Html>()
                                        }
                                    </div>

                                    <div class="post-content">
                                        {&post.content}
                                    </div>
                                </CardBody>
                            </Card>
                        </Content>
                    </SplitItem>
                </Split>
                </>
            )
        }
        None => html!(
            <>
                <Nav/>
                <Content>
                    <h1>{"Blog Post Not Found"}</h1>
                    <p>{"The blog post you're looking for doesn't exist."}</p>
                    <Button onclick={go_back}>{"Back to Blog"}</Button>
                </Content>
            </>
        ),
    }
}
