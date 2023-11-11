use std::rc::Rc;

use gloo::utils::document;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
use yew::{function_component, html, Callback, Html, Properties};
#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    is_active: bool,
}
#[function_component]
pub fn Nav(props: &Props) -> Html {
    let aria_expanded = use_state(|| false);

    let onclick = Callback::from(move |_: MouseEvent| {
        let aria_expanded = aria_expanded.clone();
        aria_expanded.set(!*aria_expanded)
    });

    html! {
        <nav class="border-gray-200 bg-gray-50 dark:bg-gray-800 dark:border-gray-700">
          <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
            <a href="#" class="flex items-center space-x-3 rtl:space-x-reverse">
                <div class="filter: invert(100%);">
                    <img src="logo.svg"  class="h-20 "/>
                </div>
                <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Cary Hawkins"}</span>
            </a>
            <button
                data-collapse-toggle="navbar-hamburger"
                type="button" class="inline-flex items-center justify-center p-2 w-10 h-10 text-sm text-gray-500 rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                aria-controls="navbar-hamburger"
                {onclick}
            >
              <span class={classes!("sr-only")}>{"Open main menu"}</span>
              <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                  <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
              </svg>
            </button>
            <div class="hidden w-full" id="navbar-hamburger">
              <ul class="flex flex-col font-medium mt-4 rounded-lg bg-gray-50 dark:bg-gray-800 dark:border-gray-700">
                <li>
                  <a href="#" class="block py-2 px-3 text-white bg-blue-700 rounded dark:bg-blue-600" aria-current="page">{"Home"}</a>
                </li>
                <li>
                  <a href="#About" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">{"About"}</a>
                </li>
                <li>
                  <a href="#Projects" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white">{"Current Projects"}</a>
                </li>
                <li>
                  <a href="#Blog" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white">{"Blog"}</a>
                </li>
              </ul>
            </div>
          </div>
        </nav>
    }
}
