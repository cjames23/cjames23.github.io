use crate::components::footer::Footer;
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
            <h2 class="flex-1 text-9xl font-extrabold text-center">{"Coming Soon!"}
            </h2>
            <Footer />
        </>
    };
}
