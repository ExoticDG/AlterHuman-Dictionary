use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Homepage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Root,

    #[at("/home")]
    Homepage,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! { <Homepage /> },
        Route::Homepage => html! { <Homepage /> },
        Route::NotFound => html! { <Homepage /> },
            }
}
