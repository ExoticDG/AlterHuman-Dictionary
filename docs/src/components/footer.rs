use std::rc::Rc;

use crate::{app::AppState, router::Route};
use yew::prelude::*;
use yew_router::components::Link;

pub enum Msg {
    ButtonClick,
    AppStateUpdate(Rc<AppState>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Footer {}

impl Component for Footer {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="h-12 bg-zinc-600 pl-10 place-content-center">
                {"Help out your local dog shelter so less dogs will get killed each year."}
            </div>
        }
    }
}


