use rand::{distributions, Rng};
use yew::prelude::*;


use crate::components::progress_delay::ProgressDelay;


#[function_component]
pub fn AuthorList() -> Html {
    html! {
        <div class="container">
            <section class="hero">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">{ "Author" }</h1>
                        <h2 class="subtitle">
                            { "Meet Silen Locatelli" }
                        </h2>
                    </div>
                </div>
            </section>
            <div class="section">
                <div class="tile is-ancestor">
                </div>
            </div>
        </div>
    }
}

