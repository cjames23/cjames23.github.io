// src/components/modal.rs
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub is_open: bool,
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ModalCom)]
pub fn modal(props: &ModalProps) -> Html {
    let on_close = props.on_close.clone();

    html! {
        if props.is_open {
            <Modal
                title={props.title.clone()}
                variant={ModalVariant::Medium}
                onclose={Some(on_close)}
                show_close={true}
            >
                { for props.children.iter() }
            </Modal>
        }
    }
}
