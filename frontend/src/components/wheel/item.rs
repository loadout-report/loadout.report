use std::fmt::{Display, Formatter};
use stylist::yew::use_style;
use yew::{Component, Context, Html};
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Item {
    pub image: String,
    pub id: i64,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub item: Item,
}

#[function_component(CarouselItem)]
pub fn carousel_item(props: &Props) -> Html {

    let style = use_style!(
        background-color: black;
        display: flex;
        justify-content: center;
        align-items: center;

        transition: transform 0.5s;
        position: relative;
        transform: scale(1);
        transform-origin: center center;
        flex-shrink: 0;

        scroll-snap-align: start;

        .active {

        }
    );
    let Props {
        item
    } = props;
    html!(
        <div class={style}>
            <img src={item.image.clone()}/>
        </div>
    )
}
