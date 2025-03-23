// src/pages/home.rs
use crate::app::ThemeContext;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;

#[function_component(Home)]
pub fn home_function() -> Html {
    // Get the theme context
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    let navigator = use_navigator().unwrap();
    let go_to_projects = Callback::from(move |_| navigator.push(&Route::Projects));

    // Define color palette based on theme
    let bg_primary = if dark_mode {
        "bg-[#3A4D39]"
    } else {
        "bg-[#ECE3CE]"
    };
    let bg_secondary = if dark_mode {
        "bg-[#4F6F52]"
    } else {
        "bg-[#739072]"
    };
    let text_primary = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#3A4D39]"
    };
    let card_bg = if dark_mode {
        "bg-[#4F6F52]"
    } else {
        "bg-[#739072]"
    };
    let card_text = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#ECE3CE]"
    };

    let first_p = "I have a rather unorthodox background as an engineer, \
                   I studied Philosophy in undergrad and pursued a life in the kitchen as a chef for over 15 years before turning towards tech. \
                   I have now been in the space of Data and Software since 2014. I started off as a Metrics Analyst at a small start up in Phoenix, AZ where I eventually became a database administrator before becoming a Data Engineer at Amazon in 2017. \
                   My passions are still evolving as I am finding myself drawn more to developer experience and build tools in more recent months.";

    html! {
        <div class={classes!("page-container", "p-8", text_primary)}>
            <Breadcrumb>
                <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
            </Breadcrumb>

            <div class="mt-6">
                <img class="h-auto max-w-full rounded-lg mb-6" src="about.jpeg" alt="Profile" />

                <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                    <CardTitle>
                        <h1 class="text-4xl font-bold tracking-tight sm:text-5xl">
                            {"I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA."}
                        </h1>
                    </CardTitle>
                    <CardBody>
                        <p class="text-lg">{first_p}</p>
                    </CardBody>
                </Card>

                <div class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "gap-6", "mt-8")}>
                    <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                        <CardTitle>{"Skills"}</CardTitle>
                        <CardBody>
                            <div class="flex flex-wrap gap-2">
                                {
                                    ["Rust", "Python", "JavaScript", "AWS", "Database Design", "Data Engineering"].iter().map(|skill| {
                                        html! {
                                            <span class={classes!(bg_primary, text_primary, "px-3", "py-1", "rounded", "text-sm")}>
                                                {skill}
                                            </span>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </CardBody>
                    </Card>

                    <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                        <CardTitle>{"Recent Projects"}</CardTitle>
                        <CardBody>
                            <ul class="list-disc list-inside space-y-2">
                                <li>{"Personal Portfolio Site (Rust/Yew)"}</li>
                                <li>{"Data Pipeline Framework (Python/AWS)"}</li>
                                <li>{"CLI Development Tools (Rust)"}</li>
                            </ul>
                        </CardBody>
                        <CardFooter>
                            <Button
            variant={ButtonVariant::Link}
            class="text-inherit hover:underline"
            onclick={go_to_projects}
        >
            {"See all projects"}
            <i class="fas fa-arrow-right ml-2"></i>
        </Button>
                        </CardFooter>
                    </Card>
                </div>
            </div>
        </div>
    }
}
