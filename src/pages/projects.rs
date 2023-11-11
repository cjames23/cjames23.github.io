use crate::components::header::Header;
use crate::components::nav::Nav;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
       <Header>
        <Nav />
           <div class="flex-auto dark">
            <div class="sm:px-8 mt-16 sm:mt-32">
            <div class="mx-auto w-full max-w-7xl lg:px-8">
                <div class="relative px-4 sm:px-8 lg:px-12">
                    <div class="mx-auto max-w-2xl lg:max-w-5xl">
                        <div class="grid grid-cols-1 gap-y-16 lg:grid-cols-2 lg:grid-rows-[auto_1fr] lg:gap-y-12">
                            <div class="pointer-events-auto mt-6 space-y-7 text-base text-black-600 dark:text-black-400">
                                <ul>
                                    <li>
                                        <a class="hover:text-red-800" href="https://www.linkedin.com/posts/apache-airflow_new-apache-airflow-provider-weve-just-released-activity-7122632024523218944-suuC" >{"Contributor to Apache Airflow."}</a>
                                    </li>
                                </ul>
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
