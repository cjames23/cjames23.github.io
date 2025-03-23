use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    html! {
        <footer class={classes!(
            "p-4",
            "flex",
            "justify-between",
            "items-center",
            "bg-gray-800",
            "text-white",
            props.class.clone()
        )}>
            <div>
                <p>{"© 2024 C. James Hawkins"}</p>
            </div>
            <div class="flex space-x-4">
                <a href="https://github.com/cjames23" class="text-white hover:text-gray-300">
                    <i class="fab fa-github text-xl"></i>
                </a>
                <a href="https://linkedin.com/in/cary-hawkins" class="text-white hover:text-gray-300">
                    <i class="fab fa-linkedin text-xl"></i>
                </a>
                <a href="mailto:contact@cjameshawkins.com" class="text-white hover:text-gray-300">
                    <i class="fas fa-envelope text-xl"></i>
                </a>
            </div>
        </footer>
    }
}
