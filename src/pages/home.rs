use yew::prelude::*;
use crate::components::{HomeHeader};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _:ComponentLink<Self>) -> Self {
        Self{}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="base">
                <HomeHeader/>
            </div>
        }
    }
}