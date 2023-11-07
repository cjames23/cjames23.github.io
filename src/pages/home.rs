use yew::prelude::*;

#[function_component(Home)]
pub fn home_function_component() -> Html {
    let message = "Coming Soon!";
    return html! {
        <>
        <div class="container mt-4">
             <div class="row justify-content-md-center">
                <p>{message}</p>
             </div>
        </div>
        </>
    };
}
