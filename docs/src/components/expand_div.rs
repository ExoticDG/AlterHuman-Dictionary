use yew::prelude::*;

pub enum Msg {
    ToggleDiv,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub children: Html,
}

pub struct ExpandDiv {
    visible: bool,
}

impl Component for ExpandDiv {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            visible: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDiv => {
                if self.visible {
                    self.visible = false;
                } else {
                    self.visible = true;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let grid_class = if self.visible {
            "grid-rows-[1fr] opacity-100"
        } else {
            "grid-rows-[0fr] opacity-0"
        };

        html! {
            <div class="flex flex-col w-4/5 mx-auto">
                <div 
                    class="border-blue-500 bg-slate-100 flex flex-row justify-between py-2 px-4"
                    onclick={ctx.link().callback(|_| Msg::ToggleDiv)}
                >   
                    <p class="text-lg font-semibold">{ctx.props().title.clone()}</p>
                    <img 
                        src={
                            if self.visible {
                                "images/icons/minus.svg"
                            } else {
                                "images/icons/plus.svg"
                            }
                        }
                        alt={"bars_icon"}
                        class="w-5"
                    />
                </div>

                <div
                    id="faqs-text-01"
                    role="region"
                    class={format!("grid overflow-hidden transition-all duration-500 ease-in-out {}", grid_class)}
                >
                    <div class="overflow-hidden bg-slate-200 p-2">
                        {ctx.props().children.clone()}
                    </div>
                </div>

            </div>
        }
    }
}


