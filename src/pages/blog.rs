use crate::components::nav::Nav;
use crate::data::blog::{BlogDb, BlogPost}; // Import the blog data structures
use patternfly_yew::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;
use yew::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    // Track which accordion items are expanded
    let expanded_state = use_state(HashSet::<String>::new);

    // Initialize the blog database with sample posts
    let blog_db = use_memo((), |_| BlogDb::new());

    let toggle = |post_id: String| {
        let state = expanded_state.clone();
        Callback::from(move |_| {
            let mut selected = (*state).clone();
            if selected.contains(&post_id) {
                selected.remove(&post_id);
            } else {
                selected.insert(post_id.clone());
            }
            state.set(selected);
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
                        <h1>{"Blog"}</h1>
                    </Content>
                    <Accordion>
                        {
                            for blog_db.get_all_posts().into_iter().map(|post| {
                                let post_id = post.id.clone();
                                let formatted_date = post.created_at.format("%B %d %Y %H:%M%P").to_string();

                                html_nested!(
        <AccordionItem
            title={formatted_date}
            onclick={toggle(post_id.clone())}
            expanded={expanded_state.contains(&post_id)}
        >
            <Card>
                <CardTitle>{&post.title}</CardTitle>
                <CardBody>
                    {&post.content}
                    <div class="text-muted mt-2">
                        {format!("By {}", post.author)}
                    </div>
                </CardBody>
                <CardFooter>
                    <div class="tag-icons">
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
                </CardFooter>
            </Card>
        </AccordionItem>
    )
                            })
                        }
                    </Accordion>
                </SplitItem>
            </Split>
            </>
        )
}
