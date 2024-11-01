use crate::components::nav::Nav;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home_function() -> Html {
    let first_p = "I have a rather unorthodox background as an engineer, \
                   I studied Philosophy in undergrad and pursued a life in the kitchen as a chef for over 15 years before turning towards tech. \
                   I have now been in the space of Data and Software since 2014. I started off as a Metrics Analyst at a small start up in Phoenix, AZ where I eventually became a database administrator before becoming a Data Engineer at Amazon in 2017. \
                   My passions are still evolving as I am finding myself drawn more to developer experience and build tools in more recent months.";
    html! {
     <>
     <Split>
     <SplitItem>
         <Nav/>
     </SplitItem>
     <SplitItem>
     <Breadcrumb>
         <BreadcrumbItem href="/">{"Home"}</BreadcrumbItem>
     </Breadcrumb>
         <img class="h-2/6" src="about.jpeg" />
             <Card>
                     <CardTitle>
                     <h1 class="text-4xl font-bold tracking-tight text-black-800 dark:text-black-100 sm:text-5xl">
                         {"I am Cary Hawkins, an Alpinist and Software Engineer from Sultan, WA."}
                     </h1>
                     </CardTitle>
                     <CardBody>
                     <p>{first_p}</p>
                     </CardBody>
             </Card>
     </SplitItem>
     </Split>
     </>
    }
}
