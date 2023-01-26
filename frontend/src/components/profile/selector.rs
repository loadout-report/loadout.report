use log::info;
use serde_derive::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::client::ExactSearchRequest;


#[derive(Clone, PartialEq, Properties)]
pub struct SelectorProps {
    pub onprofile: Callback<ExactSearchRequest>,
}

#[function_component(ProfileSelector)]
pub fn player_selector(props: &SelectorProps) -> Html {


    let onprofile = {
        let onprofile = props.onprofile.clone();
        move |player| onprofile.emit(player)
    };

    let onkeypress = {

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let target = e.target_unchecked_into::<HtmlInputElement>();
                let value: String = target.value();
                onprofile(value.as_str().into());
            }
        }
    };

    html! {
        <div>
            <h2>{"Profile Selector"}</h2>
            <input type="text" {onkeypress} />
            // <button {onclick} type="button">{"Search"}</button>
        </div>
    }
}
