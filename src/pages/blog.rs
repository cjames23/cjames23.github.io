use crate::components::nav::Nav;
use patternfly_yew::prelude::*;
use std::collections::HashSet;
use yew::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let state = use_state(HashSet::<String>::new);

    let toggle = |key: &'static str| {
        let state = state.clone();
        Callback::from(move |_| {
            let mut selected = (*state).clone();
            if selected.contains(key) {
                selected.remove(key);
            } else {
                selected.insert(key.to_string());
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
                <AccordionItem title="October 27 2024 10:42pm" onclick={toggle("first")} expanded={state.contains("first")}>
                <Card>
                    <CardTitle>{"October 27 2024"}</CardTitle>
                    <CardBody>
                    {"One thing that has been on my mind a lot lately is the future of developer productivity. I have recently been working \
                    with Gradle plugins a lot at work. This has been a big change having spent the large majority of my career as a Data Engineer. \
                    Which generally included a lot of Python. I had dabbled in Gradle only through need on a random Android project that never worked \
                    out. The more I dive deep into the world of build tools that exist at work the more I realize I have a lot of growth ahead of me. "}
                    </CardBody>
                </Card>
                </AccordionItem>
                </Accordion>
        </SplitItem>
        </Split>
        </>
    )
}
