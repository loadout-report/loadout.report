use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;
use log::{info, warn};
use rustgie_types::destiny::responses::DestinyProfileResponse;
use rustgie_types::user::UserInfoCard;
use yew::prelude::*;
use crate::client::Client;
use super::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct WrapperProps {
    pub children: Children,
}

#[function_component(FireteamWrapper)]
pub fn wrapper(props: &WrapperProps) -> Html {

    let profile: Rc<DestinyProfileResponse> = use_context::<Rc<DestinyProfileResponse>>().expect("Profile not initialised");
    let fireteam = use_state(|| None);
    let client: Rc<Client> = use_context::<Rc<Client>>().expect("Client not initialised");

    {
        let fireteam_handle = fireteam.clone();
        let profile = profile.clone();
        let client = client.clone();
        use_effect_with_deps(move |profile| {
            let profile = profile.clone();
            let client = client.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let members = profile.as_ref().clone().profile_transitory_data
                    .and_then(|x| x.data)
                    .and_then(|x| x.party_members)
                    .unwrap();

                let future = members.iter()
                    .map(|m| m.membership_id)
                    .map(|m| client.get_main_profile(m))
                    .collect::<Vec<_>>();
                let future = futures::future::join_all(future);

                let profiles = future.await;
                let profiles = profiles.into_iter()
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();
                fireteam_handle.set(Some(profiles));
            });
        }, profile.clone());
    }

    html! {
        <>
          if let Some(fireteam) = fireteam.deref() {
              <FireteamProvider fireteam={fireteam.to_owned()}>
                { for props.children.iter() }
              </FireteamProvider>
          }
        </>
    }
}
