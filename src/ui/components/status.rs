use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatusProps {
    pub status: String,
}

#[function_component(Status)]
pub fn status_component(props: &StatusProps) -> Html {
    html! {
        <p>{ format!("Status: {}", props.status) }</p>
    }
}
