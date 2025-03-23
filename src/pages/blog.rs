use crate::app::ThemeContext;
use crate::components::nav::Nav;
use crate::data::blog::{BlogDb, BlogPost};
use crate::route::Route;
use patternfly_yew::prelude::*;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let blog_db = use_memo((), |_| BlogDb::new());
    let navigator = use_navigator().unwrap();

    // Get the theme context
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    // Navigation handler
    let navigate_to_post = |post_id: String| {
        let nav = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&Route::BlogPost {
                id: post_id.clone(),
            });
        })
    };

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
                </Breadcrumb>
                <Content>
                    <h1 class={classes!("text-gray-800", "dark:text-white")}>{"Blog"}</h1>
                </Content>
                <Grid gutter={true}>
                    {
                        for blog_db.get_all_posts().into_iter().map(|post| {
                            let post_id = post.id.clone();
                            let formatted_date = post.created_at.format("%B %d %Y %H:%M%P").to_string();

                            html_nested!(
                                <GridItem cols={[12, 6, 3]}>
                                    <div class="p-2">
                                        <Card
                                            selectable=true
                                            class={classes!("bg-white", "dark:bg-gray-800", "text-gray-800", "dark:text-white")}
                                        >
                                            <CardHeader>
                                                <Link<Route>
                                                    to={Route::BlogPost { id: post_id.clone() }}
                                                    classes={classes!("text-blue-600", "dark:text-blue-400", "hover:text-red-800", "dark:hover:text-red-400")}
                                                >
                                                    <CardTitle>{&post.title}</CardTitle>
                                                </Link<Route>>
                                            </CardHeader>
                                            <CardBody>
                                                <div class={classes!("text-gray-600", "dark:text-gray-400", "mb-2")}>{formatted_date}</div>
                                                <div class="tag-icons mb-3">
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
                                            </CardBody>
                                            <CardFooter>
                                                <div class={classes!("text-gray-600", "dark:text-gray-400")}>
                                                    {format!("By {}", post.author)}
                                                </div>
                                            </CardFooter>
                                        </Card>
                                    </div>
                                </GridItem>
                            )
                        })
                    }
                </Grid>
            </SplitItem>
        </Split>
        </>
    )
}
