use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use yew::prelude::*;
use yew_hooks::prelude::*;
use serde::Deserialize;
use yew::{html, Html};

const WEAPON_TYPES: &[&str] = &["Auto Rifle", "Combat Bow", "Hand Cannon", "Pulse Rifle", "Scout Rifle", "Sidearm", "Submachine Gun",
    "Fusion Rifle", "Glaive", "Grenade Launcher", "Shotgun", "Sniper Rifle", "Trace Rifle",
    "Linear Fusion Rifle", "Machine Gun", "Rocket Launcher", "Sword"];

#[derive(Properties, PartialEq, Clone)]
pub struct WheelProps {
}

#[function_component(Wheel)]
pub fn wheel(_props: &WheelProps) -> Html {
    let state = use_async(async move { fetch_definitions().await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };
    html! {
        <div class="app">
            <h1>{"Wheel of Misfortune"}</h1>

            <p>
                <button {onclick}>{ "Load Destiny Manifest" }</button>

            </p>
            <p>
                {
                    if state.loading {
                        html! { "Loading, wait a sec..." }
                    } else {
                        html! {}
                    }
                }
            </p>
                {
                    if let Some(manifest) = &state.data {
                        let exotic_weapons = get_exotic_weapons(&manifest);
                        html! {
                            <>
                                <p>{ "Items: " }<b>{ &manifest.items.len() }</b></p>
                                <p>{ "Exotic Weapons: " }<b>{ exotic_weapons.len() }</b></p>
                                <ul class="list">
                                    {for exotic_weapons.iter().map(|item| {
                                        html! {

                                                <img src={get_image_url(item.image.clone().unwrap().as_str())} />
                                        }
                                    })}
                                </ul>
                            </>
                            }
                    } else {
                        html! {}
                    }
                }
            <p>
                    {
                        if let Some(error) = &state.error {
                            html! {error}
                        } else {
                            html! {}
                        }
                    }
            </p>
        </div>
    }
}

#[derive(Deserialize, Clone, Debug)]
struct ItemDefinition {
    #[serde(rename = "n")]
    name: Option<String>,
    #[serde(rename = "d")]
    description: Option<String>,
    #[serde(rename = "i")]
    image: Option<String>,
    #[serde(rename = "t")]
    item_type: Option<String>,
    #[serde(rename = "tT")]
    rarity: Option<u8>,
}

impl Display for ItemDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.name.clone().unwrap_or(String::new()))
    }
}

#[derive(Deserialize, Clone, Debug)]
struct DestinyDefinitions {
    items: HashMap<String, ItemDefinition>
}

fn get_exotic_weapons(manifest: &DestinyDefinitions) -> Vec<&ItemDefinition> {
    manifest.items.values()
        .filter(|i| i.rarity.unwrap_or(0) == 6)
        .filter(|i| is_weapon_type(i.item_type.clone().unwrap_or(String::new()).as_str()))
        .collect()
}

fn is_weapon_type(t: &str) -> bool {
    for tt in WEAPON_TYPES {
        if *tt == t {
            return true
        }
    }
    return false
}

fn get_image_url(i: &str) -> String {
    "https://bungie.net".clone().to_owned() + i
}

async fn fetch_definitions() -> Result<DestinyDefinitions, String> {
    let result = reqwest::get("https://api.destinytrialsreport.com/destiny2/en/DestinyDefinitions.json").await.map_err(|err| err.to_string())?;
    let definitions: DestinyDefinitions = result.json::<DestinyDefinitions>().await.map_err(|err| err.to_string())?;
    return Ok(definitions);
}
