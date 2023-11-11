use crate::components::header::Header;
use crate::components::nav::Nav;
use yew::prelude::*;

#[function_component(Home)]
pub fn home_function() -> Html {
    return html! {
        <>
            <Header>
                <Nav />
            </Header>
            <div class="md:container">
                <img src="about.jpeg" class="object-center object-fill" />
            </div>
            <h2 class="text-9xl font-extrabold text-center">{"Coming Soon!"}
            </h2>
        </>
    };
}
