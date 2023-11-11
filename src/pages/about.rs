use crate::components::header::Header;
use crate::components::nav::Nav;
use yew::prelude::*;

#[function_component(About)]
pub fn about_function() -> Html {
    html! {
       <Header>
        <Nav />
           <div class="flex-auto dark">
        <div class="sm:px-8 mt-16 sm:mt-32">
            <div class="mx-auto w-full max-w-7xl lg:px-8">
                <div class="relative px-4 sm:px-8 lg:px-12">
                    <div class="mx-auto max-w-2xl lg:max-w-5xl">
                        <div class="grid grid-cols-1 gap-y-16 lg:grid-cols-2 lg:grid-rows-[auto_1fr] lg:gap-y-12">
                            <div class="lg:pl-20">
                                <div class="max-w-xs px-2.5 lg:max-w-none">
                                    <img src="about.jpeg" />
                                </div>
                            </div>
                            <div class="lg:order-first lg:row-span-2">
                                <h1 class="text-4xl font-bold tracking-tight text-zinc-800 dark:text-zinc-100 sm:text-5xl">
                                    {"I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA."}
                                </h1>
                            </div>
                            <div class="mt-6 space-y-7 text-base text-zinc-600 dark:text-zinc-400">
                                <p>
                                    {"I have a rather unorthodox background as an engineer, I studied Philosophy in undergrad and pursued a life in the kitchen"}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
       </Header>
           }
}
