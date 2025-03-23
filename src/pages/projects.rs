use crate::app::ThemeContext;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    // Get the theme context
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    // Define color palette based on theme
    let bg_primary = if dark_mode {
        "bg-[#3A4D39]"
    } else {
        "bg-[#ECE3CE]"
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
    let link_color = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#ECE3CE]"
    };
    let link_hover = if dark_mode {
        "hover:text-[#D0D4CA]"
    } else {
        "hover:text-[#D0D4CA]"
    };

    html! {
        <div class={classes!("page-container", "p-8", text_primary)}>
            <Breadcrumb>
                <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                <BreadcrumbItem href="/projects">{"Projects"}</BreadcrumbItem>
            </Breadcrumb>

            <h1 class="text-3xl font-bold mt-4 mb-6">{"Projects"}</h1>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div class="p-2">
                    <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                        <CardTitle>{"Contributor to Apache Airflow"}</CardTitle>
                        <CardBody>
                            <a class={classes!(link_color, link_hover, "underline")}
                               href="https://www.linkedin.com/posts/apache-airflow_new-apache-airflow-provider-weve-just-released-activity-7122632024523218944-suuC">
                                {"Released the first pieces to an Open Search Airflow Provider"}
                            </a>
                        </CardBody>
                    </Card>
                </div>

                <div class="p-2">
                    <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                        <CardTitle>{"Maintainer of Pypi Proxy Poetry Plugin"}</CardTitle>
                        <CardBody>
                            <a class={classes!(link_color, link_hover, "underline")}
                               href="https://github.com/chadac/poetry-plugin-pypi-proxy">
                                {"Maintain a plugin that was developed to handle proxy repos for pypi including auth."}
                            </a>
                        </CardBody>
                    </Card>
                </div>

                <div class="p-2">
                    <Card class={classes!(card_bg, card_text, "shadow-md", "rounded-lg")}>
                        <CardTitle>{"Personal Portfolio Site"}</CardTitle>
                        <CardBody>
                            <p>{"Built with Rust and Yew framework, showcasing modern web development with WASM"}</p>
                            <a class={classes!(link_color, link_hover, "underline", "mt-2", "block")}
                               href="https://github.com/cjames23/portfolio">
                                {"View on GitHub"}
                            </a>
                        </CardBody>
                    </Card>
                </div>
            </div>
        </div>
    }
}
