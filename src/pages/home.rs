use crate::components::nav::Nav;
use yew::prelude::*;

#[function_component(Home)]
pub fn home_function() -> Html {
    let first_p = "I have a rather unorthodox background as an engineer, \
                   I studied Philosophy in undergrad and pursued a life in the kitchen as a chef for over 15 years before turning towards tech. \
                   I have now been in the space of Data and Software since 2014. I started off as a Metrics Analyst at a small start up in Phoenix, AZ where I eventually became a database administrator before becoming a Data Engineer at Amazon in 2017. \
                   My passions are still evolving as I am finding myself drawn more to developer experience and build tools in more recent months.";
    html! {
    <>
        <div class="columns-2">
        <Nav/>
        <div class="container-xl h-fit columns-xl">
        <div class="container mx-auto flex-auto flex-row">
        <div class="sm:px-12 mt-16 sm:mt-32">
        <div class="mx-auto w-full max-w-7xl lg:px-8">
            <div class="relative px-4 sm:px-8 lg:px-12">
                <div class="mx-auto max-w-2xl lg:max-w-5xl">
                    <div class="lg:pl-20">
                            <div class="max-w-xs px-2.5 lg:max-w-none">
                                <img src="about.jpeg" />
                            </div>
                        </div>
                    <div class="flex-1 grid grid-cols-1 gap-y-16 lg:grid-cols-2 lg:grid-rows-[auto_1fr] lg:gap-y-12">

                        <div class="lg:order-first lg:row-span-2">
                            <h1 class="text-4xl font-bold tracking-tight text-black-800 dark:text-black-100 sm:text-5xl">
                                {"I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA."}
                            </h1>
                        </div>
                        <div class="mt-6 space-y-7 text-base text-black-600 dark:text-black-400">
                            <p class="flex-auto">
                                {first_p}
                            </p>
                        </div>

                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    </>
       }
}
