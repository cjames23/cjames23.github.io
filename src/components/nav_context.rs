// src/components/nav_context.rs
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NavContext {
    pub collapsed: bool,
    pub toggle_collapsed: Callback<()>,
}
