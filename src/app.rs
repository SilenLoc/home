use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::pages::author_list::AuthorList;
use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::post_list::PostList;
use crate::pages::cv::CV;
use crate::pages::apps::Apps;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/app")]
    Apps,
    #[at("/cv")]
    CV,
    #[at("/posts")]
    Posts,
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Made by " }
                    <a href="https://github.com/SilenLoc">{ "Silen" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
                    <div class="content has-text-centered">
                    { "Made by " }
                    <a href="https://github.com/SilenLoc">{ "Silen" }</a>
                </div>
            </footer>
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Posts => {
            html! { <PostList /> }
        }
        Route::Authors => {
            html! { <AuthorList /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::CV => {
            html! { <CV /> }
        }
        Route::Apps => {
            html! { <Apps /> }
        }
    }
}
