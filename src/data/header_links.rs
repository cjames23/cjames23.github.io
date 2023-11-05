pub struct URLRoute {
    pub name: &'static str,
    pub route: &'static str,
}

pub static HEADER_LINKS: [URLRoute; 2] = [
    URLRoute {
        name: "About",
        route: "/#about",
    },
    URLRoute {
        name: "Blog",
        route: "/#blog",
    },
];
