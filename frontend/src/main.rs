mod app;
mod routes;
mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
