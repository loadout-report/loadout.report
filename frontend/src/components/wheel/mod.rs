use std::rc::Rc;
use log::info;
use rustgie::types::destiny::responses::DestinyProfileResponse;
use stylist::style;
use stylist::yew::use_style;
use yew::prelude::*;
use yew::{utils};
use yew::{Children};
use yew::suspense::{use_future, UseFutureHandle};
use data::api::manifest::model::Item;
use roll::Roll;

pub mod roll;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RollOption {
    pub name: String,
    pub icon: String,
}

impl RollOption {
    fn new(name: String, icon: String) -> Self {
        RollOption {
            name,
            icon,
        }
    }
}

async fn get_roll_options() -> Result<Vec<RollOption>, gloo_net::Error> {
    let mut special = vec![
        RollOption::new("Ghostbusters".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034233774503907480/unknown.png".to_string()),
        RollOption::new("Space Cowboys".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034226634376630355/unknown.png?width=457&height=676".to_string()),
        RollOption::new("Rat Pack".to_string(), "".to_string()),
        RollOption::new("Siva Crisis 2".to_string(), "".to_string()),
        RollOption::new("Drifter would be proud".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034228384475123712/unknown.png".to_string()),
        RollOption::new("Arrow to the Knee".to_string(), "https://media.discordapp.net/attachments/907098694992687144/1034217287030419456/unknown.png?width=886&height=676".to_string())
    ];
    let mut fetched_exotics: Vec<RollOption> = fetch_exotics()
        .await?
        .iter()
        .map(|item| RollOption::new(item.label.clone(), item.icon.clone()))
        .collect();
    special.append(&mut fetched_exotics);
    // info!("fetched exotics: {:?}", special);
    Ok(special)
}


async fn fetch_exotics() -> Result<Vec<Item>, gloo_net::Error> {
    info!("fetching exotics");
    let result = gloo_net::http::Request::get("http://localhost:58080/items?rarity=6&category=1").send().await?;
    let definitions: Vec<Item> = result.json::<Vec<Item>>().await?;
    return Ok(definitions);
}


#[derive(Clone, PartialEq, Eq, Debug, Properties)]
pub struct LoadedWheelProps {
    // pub players: Vec<String>,
}


#[function_component(LoadedWheel)]
pub fn loaded_wheel(props: &LoadedWheelProps) -> HtmlResult {
    let roll_options: UseFutureHandle<Result<Vec<RollOption>, gloo_net::Error>> = use_future(|| async {
        let roll_options = get_roll_options().await?;
        // info!("roll options: {:?}", roll_options);
        Ok(roll_options)
    })?;

    let fireteam: Rc<Vec<DestinyProfileResponse>> = use_context::<Rc<Vec<DestinyProfileResponse>>>().unwrap();
    let roll_options = match *roll_options {
        Ok(ref res) => html! {
            <>
              {
                  fireteam.iter().map(|player| {
                    let player_name = player.profile.clone()
                        .and_then(|profile| profile.data)
                        .and_then(|data| data.user_info)
                        .and_then(|info| info.display_name)
                        .unwrap();
                    html! {
                      <div key={player_name.to_owned()}>
                        <p>{player_name.to_owned()}</p>
                        <Roll roll_options={res.to_owned()} />
                      </div>
                    }
                  }).collect::<Html>()
              }
            </>
        },
        Err(ref failure) => failure.to_string().into()
    };

    Ok(html! {
        <>
            {roll_options}
        </>
    })
}

#[derive(Clone, PartialEq, Eq, Debug, Properties)]
struct RollOptionProviderProps {
    roll_options: Vec<RollOption>,
}

#[function_component(RollOptionProvider)]
fn roll_option_provider(props: &RollOptionProviderProps) -> Html {
    info!("roll options: {:?}", props.roll_options);
    let options = use_memo(|_| {
        props.roll_options.to_owned()
    }, ());

    html! {
        // <ContextProvider<Rc<Vec<RollOption>>> context={options}>
            //<Roll roll_options={props.roll_options} />
        // </ContextProvider<Rc<Vec<RollOption>>>>
    }
}
