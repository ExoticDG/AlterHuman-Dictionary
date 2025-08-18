use std::rc::Rc;

use crate::{app::AppState, router::Route};
use yew::prelude::*;
use yew_router::components::Link;

pub enum Msg {
    CloseMe,
    AppStateUpdate(Rc<AppState>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Menu {
    app_state: Rc<AppState>,
    _app_listener: ContextHandle<Rc<AppState>>,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // hide the header if the menu is visible
        let (app_state, _app_listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(Msg::AppStateUpdate))
            .expect("context to be set");

        Self {
            app_state,
            _app_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        log::debug!("menu was updated");

        match msg {
            Msg::CloseMe => {
                log::debug!("button was clicked");
                self.app_state.toggle_menu.emit(());
                false
            },
            Msg::AppStateUpdate(s) => {
                log::debug!("menu update");
                true
            }
        }
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        log::debug!("menu view {}", self.app_state.show_menu);

        html! {
            <div class="z-40 fixed top-0 left-0 bottom-0 w-56 bg-zinc-900 text-zinc-200 flex flex-col">
                <img 
                    src={"images/icons/bars.svg"}
                    alt={"bars_icon"}
                    class="w-12"
                    onclick={ctx.link().callback(|_| Msg::CloseMe)}
                    />

                <div onclick={ctx.link().callback(|_| Msg::CloseMe)}><Link<Route> to={Route::Homepage}>{"Home"}</Link<Route>></div>
            </div>
        }
    }
}


