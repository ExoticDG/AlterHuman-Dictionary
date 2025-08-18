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

pub struct Header {
    app_state: Rc<AppState>,
    _app_listener: ContextHandle<Rc<AppState>>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _app_listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(Msg::AppStateUpdate))
            .expect("context to be set");

        Self {
            app_state,
            _app_listener
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick => {
                log::debug!("button was clicked");
                self.app_state.toggle_menu.emit(());
                false
            },
            Msg::AppStateUpdate(_) => {
                true
            }
        }
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="z-40 fixed top-0 left-0 right-0 h-12 bg-zinc-600 flex flex-row text-3xl gap-10 text-zinc-200">
                <img 
                    src={"images/icons/bars.svg"}
                    alt={"bars_icon"}
                    class="w-12"
                    onclick={ctx.link().callback(|_| Msg::ButtonClick)}
                    />
                <Link<Route> to={Route::Homepage}>{"Protect the puppy!"}</Link<Route>>
            </div>
        }
    }
}


