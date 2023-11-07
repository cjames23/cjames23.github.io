use yew::prelude::*;
use crate::components::nav::Nav;

#[function_component(Home)]
pub fn home_function_component() -> Html {
    let message = "Coming Soon!";
    return html! {
        <>
        <div class="container mt-4">
            <Nav />
             <div class="row justify-content-md-center">
                <p>{message}</p>
             </div>
        </div>
        </>
    };
}
