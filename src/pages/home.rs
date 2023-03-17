use yew::prelude::*;

#[function_component]
fn InfoTiles() -> Html {
    html! {
        <>
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <p class="title">{ "What is this?" }</p>
                    <p class="subtitle">{ "Find out!" }</p>

                    <div class="content">
                        {r#"
                            You will find here my apps, posts, my CV and some stuff about me
                            "#}
                    </div>
                </div>
            </div>

            <div class="tile is-parent">
                <div class="tile is-child box">
                    <p class="title">{ "Who am I?" }</p>
                    <div class="content">
                        { "I am Silen" }
                        <sup>{ 64 }</sup>
                        { " I am working tirelessly to bring you the low-effort content you crave for." }
                        <br />
                        {r#"
                                This is written in rust, please learn rust, I am not good at it and did this.
                            "#}
                    </div>
                </div>
            </div>
        </>
    }
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="base tile is-ancestor is-vertical">

            <div class="tile is-child">
                <figure class="image">
                    <img alt="some picture" src="https://source.unsplash.com/random/1200x400/?yew" />
                </figure>
            </div>

            <div class="tile is-parent container">
                <InfoTiles />
            </div>
        </div>
    }
}
