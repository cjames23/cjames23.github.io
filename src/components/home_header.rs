use yew::prelude::*;

fn about_content() -> Html {
    html! {
        <div>
            <p> { "My name is Cary Hawkins and I am a Senior Data Engineer at Amazon."}
            <br/>
                {"Feel free to look around here, I will be keeping this as both a blog and work demonstration"}
            </p>
        </div>
    }
}

#[derive(Clone, Debug)]
pub struct HomeHeader{
    link: ComponentLink<Self>,
}

impl Component for HomeHeader {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header_links = HEADER_LINKS
            .iter()
            .map(|v| {
                html! {
                    <div className="-m-1.5 p-1.5 rounded-md font-bold first-letter:uppercase hover:transition-colors hover:duration-300 focus:outline-none focus-visible:ring-2 focus-visible:ring-orange-500 sm:hover:text-orange-500 text-neutral-100">
                    <NavItem
                        <a href={v.route}>
                        {v.name}
                    </a>
                    />
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <header className="fixed top-0 z-50 hidden w-full bg-neutral-900/50 p-4 backdrop-blur sm:block">
                <nav className="flex justify-center gap-x-8">
                {header_links}
                </nav>
            </header>

        }
    }
}