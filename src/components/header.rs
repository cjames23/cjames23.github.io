use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="flex items-center p-4 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 shadow-sm">
            <img src="logo.svg" alt="Logo" class="h-10 w-10 mr-3" />
            <h1 class="text-xl font-bold text-gray-800 dark:text-white">{"C. James Hawkins"}</h1>
        </div>
    }
}
