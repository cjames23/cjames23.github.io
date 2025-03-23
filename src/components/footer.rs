// src/components/footer.rs
use yew::prelude::*;
use crate::app::ThemeContext;

#[function_component(Footer)]
pub fn footer() -> Html {
    let theme_context = use_context::<ThemeContext>().expect("Theme context not found");

    html! {
        <footer class="footer mt-auto py-4 border-t dark:border-gray-700">
            <div class="container mx-auto px-4 md:px-8">
                <div class="flex flex-col md:flex-row justify-between items-center">
                    <div class="mb-4 md:mb-0">
                        <p class="text-gray-600 dark:text-gray-400">
                            {"© 2025 Cary Hawkins. All rights reserved."}
                        </p>
                    </div>
                    <div class="flex space-x-4">
                        <a href="https://github.com/cjames23" class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">
                            <i class="fab fa-github text-xl"></i>
                        </a>
                        <a href="https://twitter.com/" class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">
                            <i class="fab fa-twitter text-xl"></i>
                        </a>
                        <a href="https://linkedin.com/" class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">
                            <i class="fab fa-linkedin text-xl"></i>
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}