use std::collections::HashSet;
use patternfly_yew::core::WithBreakpointExt;
use patternfly_yew::prelude::{Accordion, AccordionItem, Content, Flex, FlexItem, FlexModifier};
use yew::prelude::*;
use crate::components::nav::Nav;

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
            <Flex>
            <Flex modifiers={[FlexModifier::Column.lg()]}>
                <FlexItem>
                <Nav/>
                </FlexItem>
            </Flex>
            </Flex>
            <Flex>
            <Flex modifiers={[FlexModifier::Column.lg()]}>
            <FlexItem>
                <Accordion>
                    <AccordionItem title="October 27 2024 10:42pm" onclick={toggle("first")} expanded={state.contains("first")}>
                        {"One thing that has been on my mind a lot lately is the future of developer productivity. I have recently been working \
                        with Gradle plugins a lot at work. This has been a big change having spent the large majority of my career as a Data Engineer. \
                        Which generally included a lot of Python. I had dabbled in Gradle only through need on a random Android project that never worked \
                        out. The more I dive deep into the world of build tools that exist at work the more I realize I have a lot of growth ahead of me. "}
                    </AccordionItem>
                    </Accordion>
            </FlexItem>
            </Flex>
            </Flex>
            </>
        )
}
