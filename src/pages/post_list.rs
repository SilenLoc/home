use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn PostList() -> Html {
        html! {
        <div class="section container">
            <h1 class="title">{ "Posts" }</h1>
            <h2 class="subtitle">{ "All of my writing is here" }</h2>
        </div>
    }

}

