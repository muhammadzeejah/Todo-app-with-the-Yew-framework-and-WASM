use yew::prelude::*;
mod components;
use components::app::App;

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <App />
    }
}