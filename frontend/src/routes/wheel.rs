use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::components::wheel::{CarouselWheel, CarouselItem};
use js_sys::Math::{floor, random};

use yew::html;
use data::api::manifest::model::Item;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct WheelProps {
}

#[function_component(Wheel)]
pub fn wheel(_props: &WheelProps) -> Html {
    let exotics = use_state(Vec::new);
    let exotic = use_state(|| None);
    {
        let exotics = exotics.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_exotics: Vec<Item> = reqwest::get("http://localhost:8080/items?rarity=6&category=1")
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                exotics.set(fetched_exotics);
            });
            || ()
        }, ());
    }

    let exotic_callback_handle = exotic.clone();
    let onclick = Callback::from(move |_| {
        let exotic = exotic_callback_handle.clone();
        let random = random();
        let roll = floor(random * exotics.len() as f64) as usize;
        let roll = exotics.get(roll);
        exotic.set(Some(roll.unwrap().clone()));
    });
    html! {
        <div class="app">
            <h1>{"Wheel of Misfortune"}</h1>
            <CarouselWheel></CarouselWheel>
            <p>
                <button {onclick}>{"Roll a gun"}</button>
                {
                    if let Some(roll) = (*exotic).clone() {
                        html!(
                            <img src={roll.icon} alt={"exotic"} />
                        )
                    } else {
                        html!(
                            <div><p></p></div>
                        )
                    }
                }
            </p>

        </div>
    }
}

async fn fetch_exotics() -> Result<Vec<Item>, String> {
    let result = reqwest::get("http://localhost:8080/items?rarity=6&category=1").await.map_err(|err| err.to_string())?;
    let definitions: Vec<Item> = result.json::<Vec<Item>>().await.map_err(|err| err.to_string())?;
    return Ok(definitions);
}
