// src/components/footer.rs
use crate::app::ThemeContext;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let theme_context = use_context::<ThemeContext>().expect("ThemeContext not found");
    let dark_mode = theme_context.dark_mode;

    // Define color palette based on theme
    let bg_footer = if dark_mode {
        "bg-[#4F6F52]"
    } else {
        "bg-[#739072]"
    };
    let text_color = if dark_mode {
        "text-[#ECE3CE]"
    } else {
        "text-[#ECE3CE]"
    };

    html! {
        <footer class={classes!(
            "p-4",
            "flex",
            "justify-between",
            "items-center",
            "mt-auto", // Pushes the footer to the bottom of flex container
            bg_footer,
            text_color,
            props.class.clone()
        )}>
            <div>
                <p>{"© 2024 C. James Hawkins"}</p>
            </div>
            <div class="flex space-x-4">
                <a href="https://github.com/cjames23" class="hover:opacity-80 transition-opacity">
                    <i class="fab fa-github text-xl"></i>
                </a>
                <a href="https://linkedin.com/in/cary-hawkins" class="hover:opacity-80 transition-opacity">
                    <i class="fab fa-linkedin text-xl"></i>
                </a>
                <a href="mailto:contact@cjameshawkins.com" class="hover:opacity-80 transition-opacity">
                    <i class="fas fa-envelope text-xl"></i>
                </a>
            </div>
        </footer>
    }
}
