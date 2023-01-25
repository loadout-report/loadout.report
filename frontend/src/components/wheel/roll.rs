use yew::{function_component, html, Html};

#[function_component(Roll)]
pub fn roll_component() -> Html {


    html!(
        <>
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
                            <div><p>{"Not rolled yet"}</p></div>
                        )
                    }
                }
                </>
    )
}
