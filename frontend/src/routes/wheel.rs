use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::components::wheel::{CarouselWheel, CarouselItem};
use js_sys::Math::{floor, random};

use yew::html;
use data::api::manifest::model::Item;

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct WheelProps {
}

#[derive(Clone)]
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
            <CarouselWheel></CarouselWheel>
            <p>
                <button {onclick}>{"Roll a gun"}</button>
                {
                    if let Some(roll) = (*exotic).clone() {
                        html!(
                            <div>
                                <img style="width: 96px; height: 96px" src={roll.icon.clone()} alt={"exotic"} />
                                <h3>{roll.name.clone()}</h3>
                            </div>
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
