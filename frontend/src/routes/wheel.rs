use yew::prelude::*;
use std::rc::Rc;
use yew_hooks::prelude::*;
use crate::components::wheel::{CarouselWheel, CarouselItem};
use crate::components::wheel::roll::Roll;
use js_sys::Math::{floor, random};

use yew::html;
use yew::suspense::{Suspension, SuspensionResult};
use data::api::manifest::model::Item;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct WheelProps {
}

#[derive(Clone, PartialEq, Eq)]
struct RollOption {
    name: String,
    icon: String,
}

impl RollOption {
    fn new(name: String, icon: String) -> Self {
        RollOption {
            name,
            icon,
        }
    }
}



fn use_items() -> SuspensionResult<Vec<RollOption>> {
    match load_items() {
        Some(items) => Ok(items),
        None => {
            let (s, handle) = Suspension::new();
            on_load_items_complete(move || {handle.resume();});
            Err(s)
        }
    }
}

#[function_component(Wheel)]
pub fn wheel(_props: &WheelProps) -> Html {
    let exotics = use_state(Vec::new);
    let exotic = use_state(|| None);
    {
        let exotics = exotics.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut special = vec![
                RollOption::new("Ghostbusters".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034233774503907480/unknown.png".to_string()),
                RollOption::new("Space Cowboys".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034226634376630355/unknown.png?width=457&height=676".to_string()),
                RollOption::new("Rat Pack".to_string(), "".to_string()),
                RollOption::new("Siva Crisis 2".to_string(), "".to_string()),
                RollOption::new("Drifter would be proud".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034228384475123712/unknown.png".to_string()),
                RollOption::new("Arrow to the Knee".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034217287030419456/unknown.png?width=886&height=676".to_string())
                    ];
                let mut fetched_exotics: Vec<RollOption> = reqwest::get("http://localhost:8080/items?rarity=6&category=1")
                    .await
                    .unwrap()
                    .json::<Vec<Item>>()
                    .await
                    .unwrap()
                    .iter()
                    .map(|item| RollOption::new(item.label.clone(), item.icon.clone()))
                    .collect();
                special.append(&mut fetched_exotics);
                exotics.set(special);
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
        let roll = (*roll.unwrap()).clone();
        exotic.set(Some(roll));
    });
    html! {
        <div class="app">
            <h1>{"Wheel of Misfortune"}</h1>
            <ContextProvider<Rc<Vec<RollOption>>> context={exotics}>
                <Roll />
            </ContextProvider<Rc<Vec<RollOption>>>>
        </div>
    }
}


async fn fetch_exotics() -> Result<Vec<Item>, String> {
    let result = reqwest::get("http://localhost:8080/items?rarity=6&category=1").await.map_err(|err| err.to_string())?;
    let definitions: Vec<Item> = result.json::<Vec<Item>>().await.map_err(|err| err.to_string())?;
    return Ok(definitions);
}
