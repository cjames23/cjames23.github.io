use crate::app::ThemeContext;
use crate::components::nav::Nav;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    // Get the theme context
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");
    let dark_mode = theme_context.dark_mode;

    html! {
    <>
    <Grid gutter=true>
        <GridItem cols={[2]} rows={[12]}>
            <Nav />
        </GridItem>
        <GridItem cols={[10]} rows={[1]}>
            <Breadcrumb>
                <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
                <BreadcrumbItem href="/projects">{"Projects"}</BreadcrumbItem>
            </Breadcrumb>
        </GridItem>
        <GridItem cols={[10]} rows={[4]}>
            <Content>
                <h1 class={classes!("text-gray-800", "dark:text-white")}>{"Projects"}</h1>
            </Content>
        </GridItem>
        <GridItem cols={[4]} rows={[4]}>
            <div class="p-2">
                <Card class={classes!("bg-white", "dark:bg-gray-800", "text-gray-800", "dark:text-white")}>
                    <CardTitle>{"Contributor to Apache Airflow."}</CardTitle>
                    <CardBody>
                        <a class={classes!("text-blue-600", "dark:text-blue-400", "hover:text-red-800", "dark:hover:text-red-400")}
                           href="https://www.linkedin.com/posts/apache-airflow_new-apache-airflow-provider-weve-just-released-activity-7122632024523218944-suuC">
                        {"Released the first pieces to an Open Search Airflow Provider"}</a>
                    </CardBody>
                </Card>
            </div>
        </GridItem>
        <GridItem cols={[4]} rows={[4]}>
            <div class="p-2">
                <Card class={classes!("bg-white", "dark:bg-gray-800", "text-gray-800", "dark:text-white")}>
                    <CardTitle>{"Maintainer of Pypi Proxy Poetry Plugin "}</CardTitle>
                    <CardBody>
                        <a class={classes!("text-blue-600", "dark:text-blue-400", "hover:text-red-800", "dark:hover:text-red-400")}
                           href="https://github.com/chadac/poetry-plugin-pypi-proxy">
                        {"Maintain a plugin that was developed to handle proxy repos for pypi including auth."}</a>
                    </CardBody>
                </Card>
            </div>
        </GridItem>
    </Grid>
    </>
    }
}
