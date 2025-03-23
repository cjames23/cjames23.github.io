// src/pages/home.rs
use crate::app::ThemeContext;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home_function() -> Html {
    // Get the theme context
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    let first_p = "I have a rather unorthodox background as an engineer, \
                   I studied Philosophy in undergrad and pursued a life in the kitchen as a chef for over 15 years before turning towards tech. \
                   I have now been in the space of Data and Software since 2014. I started off as a Metrics Analyst at a small start up in Phoenix, AZ where I eventually became a database administrator before becoming a Data Engineer at Amazon in 2017. \
                   My passions are still evolving as I am finding myself drawn more to developer experience and build tools in more recent months.";

    html! {
        <div class="page-container p-8">
            <Breadcrumb>
                <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
            </Breadcrumb>

            <div class="mt-6">
                <img class="h-auto max-w-full rounded-lg mb-6" src="about.jpeg" alt="Profile" />

                <Card class={classes!("bg-white", "dark:bg-gray-800", "text-gray-800", "dark:text-white")}>
                    <CardTitle>
                        <h1 class={classes!("text-4xl", "font-bold", "tracking-tight", "text-gray-800", "dark:text-white", "sm:text-5xl")}>
                            {"I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA."}
                        </h1>
                    </CardTitle>
                    <CardBody>
                        <p class={classes!("text-lg", "text-gray-700", "dark:text-gray-300")}>{first_p}</p>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}
