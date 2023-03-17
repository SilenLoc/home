use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Apps() -> Html {
        html! {
        <div class="section container">
            <h1 class="title">{ "You can find my apps here:" }</h1>
            <a href="https://silenloc.github.io/omnis-vanitas-web/">{ "Apps" }</a>
        </div>
    }

}

