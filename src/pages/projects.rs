use crate::components::nav::Nav;
use yew::prelude::*;
use patternfly_yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <>
        <Grid>
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
            <h1>{"Projects"}</h1>
        </Content>
        </GridItem>
        <GridItem cols={[4]} rows={[4]}>
        <Card>
        <CardTitle>{"Contributor to Apache Airflow."}</CardTitle>
        <CardBody>
        <a class="hover:text-red-800" href="https://www.linkedin.com/posts/apache-airflow_new-apache-airflow-provider-weve-just-released-activity-7122632024523218944-suuC">
        {"Released the first pieces to an Open Search Airflow Provider"}</a>
        </CardBody>
        </Card>
        </GridItem>
        <GridItem cols={[4]} rows={[4]}>
        <Card>
        <CardTitle>{"Maintainer of Pypi Proxy Poetry Plugin "}</CardTitle>
        <CardBody>
        <a class="hover:text-red-800" href="https://github.com/chadac/poetry-plugin-pypi-proxy" >
        {"Maintain a plugin that was developed to handle proxy repos for pypi including auth."}</a>
        </CardBody>
        </Card>
        </GridItem>
        </Grid>

        </>
           }
}
