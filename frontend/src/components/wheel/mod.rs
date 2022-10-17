use stylist::style;
use stylist::yew::use_style;
use yew::prelude::*;
use yew::{utils};
use yew::{Children};

pub mod item;

pub use item::CarouselItem;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    pub onrollfinish_signal: Callback<usize>,
    #[prop_or(true)]
    pub loading: bool,
    #[prop_or_default]
    pub children: Children,
}

pub enum Message {
    RollFinishEvent(usize)
}

#[function_component(CarouselWheel)]
pub fn carousel_wheel(props: &Props) -> Html {
    let container_style = use_style!(
        overflow: hidden;
        width: 480px;
        text-align: center;
    );
    let style = use_style!(
        background-color: pink;
        display: flex;
        overflow-x: auto;
        scroll-snap-type: x mandatory;
        scroll-behavior: smooth;
        -webkit-overflow-scrolling: touch;

        -ms-overflow-style: none;
        scrollbar-width: none;
        ::-webkit-scrollbar {
            display: none;
        }
    );

    let image = String::from("https://www.bungie.net/common/destiny2_content/icons/e4b1e51834802439cb342a439b0079cf.jpg");

    let items = (1..=20).map(|id| item::Item {
        image: image.clone(),
        id
    }).collect::<Vec<_>>();

    html!(
        <div class={container_style}>
            <div class={style}>
                {
                    for items.iter().map(|item| {
                        html!(
                            <CarouselItem key={format!("{}", item.id)} item={item.clone()} />
                        )
                    })
                }
            </div>
        </div>
    )
}

