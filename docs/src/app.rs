use std::rc::Rc;

use crate::{
    components::{Footer, Header, Menu}, 
    router::{switch, Route}
};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

// /// This is the shared state between the parent and child components.
#[derive(Clone, PartialEq, Default)]
pub struct AppState {
    pub show_menu: bool,
    pub show_header: bool,
    pub toggle_menu: Callback<()>
}

pub enum Msg {
    ToggleMenuState(()),
}

pub struct App {
    app_state: Rc<AppState>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let app_state = Rc::new(AppState {
            show_header: true,
            show_menu: false,
            toggle_menu: ctx.link().callback(Msg::ToggleMenuState)
        });

        Self {
            app_state
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenuState(()) => {
                // arbitrary code
                //...
                let shared_app_state = Rc::make_mut(&mut self.app_state);
                if shared_app_state.show_menu == true {
                    shared_app_state.show_menu = false;
                    shared_app_state.show_header = true;
                    log::debug!("setting show menu to false");    
                } else {
                    shared_app_state.show_menu = true;
                    shared_app_state.show_header = false;
                    log::debug!("setting show menu to true");
                }
                log::debug!("app toggle menu");
                true
            }

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        
        let app_state = self.app_state.clone();
        let show_header = app_state.show_header;
        let show_menu = app_state.show_menu;

        html! {
            <ContextProvider<Rc<AppState>> context={app_state}>
                <BrowserRouter>
                    <header>
                    if show_header {
                        <Header/>
                    }
                    </header>
                    <nav>
                    if show_menu {
                        <Menu/>
                    }
                    </nav>
                    
                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                    
                    <Footer/>
                </BrowserRouter>
            </ContextProvider<Rc<AppState>>>
        }
    }
}
