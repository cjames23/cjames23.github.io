use crate::components::header::Header;
use crate::components::nav::Nav;
use yew::prelude::*;

#[function_component(Home)]
pub fn home_function() -> Html {
    let message = "Coming Soon!";
    return html! {
        <>
            <div class="flex w-full">
                <Header>
                    <Nav/>
                </Header>
            </div>
        </>
    };
}
