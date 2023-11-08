use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <div class="top-0 z-10 h-16 pt-6" style="position:var(--header-position)">
            <div class="sm:px-8 top-[var(--header-top,theme(spacing.6))] w-full" style="position:var(--header-inner-position)">
                <div class="mx-auto w-full max-w-7xl lg:px-8">
                    <div class="relative px-4 sm:px-8 lg:px-12">
                        <div class="mx-auto max-w-2xl lg:max-w-5xl">
                            <div class="relative flex gap-4">
                            <div class="flex flex-1"></div>
                                <div class="flex flex-1 justify-end md:justify-center">
                                    <nav class="pointer-events-auto hidden md:block">
                                        <ul class="flex rounded-full bg-white/90 px-3 text-sm font-medium text-zinc-800 shadow-lg shadow-zinc-800/5 ring-1 ring-zinc-900/5 backdrop-blur dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10">
                                            <li>
                                                <a class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400" href="/about">{"About"}</a>
                                            </li>
                                            <li>
                                                <a class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400" href="/article">{"Articles"}</a>
                                            </li>
                                            <li>
                                                <a class="relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400" href="/projects">{"Projects"}</a>
                                            </li>
                                        </ul>
                                    </nav>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
