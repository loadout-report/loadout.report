use yew::prelude::*;
use std::rc::Rc;
use rustgie_types::destiny::responses::DestinyProfileResponse;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ProviderProps {
    pub children: Children,
    pub profile: DestinyProfileResponse,
}

#[function_component(ProfileProvider)]
pub fn provider(props: &ProviderProps) -> Html {
    let profile = use_memo(
        |profile| profile.to_owned(),
        props.profile.to_owned()
    );

    html! {
        <ContextProvider<Rc<Profile>> context={profile}>
          { for props.children.iter() }
        </ContextProvider<Rc<Profile>>>
    }
}
