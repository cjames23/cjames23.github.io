use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    html! {
        <div class="md:container md:mx-auto dark">
            <div class="flex w-full max-w-7xl lg:px-8">
                <div class="w-full bg-white ring-1 ring-zinc-100 dark:bg-zinc-900 dark:ring-zinc-300/20">
                </div>
            </div>
            <div class="relative flex w-full flex-col">
                <header class="pointer-events-none relative z-50 flex flex-none flex-col" style="height:var(--header-height);margin-bottom:var(--header-mb)">
                    <div class="order-last mt-[calc(theme(spacing.16)-theme(spacing.3))]">
                    </div>
                    { for props.children.iter() }
                </header>
            </div>
        </div>
    }
}
