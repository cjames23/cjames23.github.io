#![recursion_limit = "640"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app_router;
mod pages;
mod routes;
mod components;
mod data;


use app_router::AppRouter;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <AppRouter/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app(){
    App::<Model>::new().mount_to_body();
}
