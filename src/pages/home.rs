use yew::prelude::*;
use crate::components::{HomeHeader};

pub struct Home;

impl Component for Home {
    fn create(_: Self::Properties, _:ComponentLink<Self>) -> Self {
        Self{}
    }

    fn view(&self) -> Html {
        html! {
            <div class="base">
                <HomeHeader/>
            </div>
        }
    }
}