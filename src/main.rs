use yew::prelude::*;

use cjames_web::pages::home::Home;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Home/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
