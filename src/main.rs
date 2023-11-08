use cjames_web::app::App;
use cjames_web::pages::home::Home;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
