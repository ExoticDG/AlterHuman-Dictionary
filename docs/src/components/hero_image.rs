use yew::prelude::*;

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
}

pub struct HeroImage {}

impl Component for HeroImage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="basis-[900px] bg-cover bg-top bg-fixed relative" style="background-image: url(/images/Background_Nova.jpg)">
                <div class="absolute left-[300px] top-[240px] text-4xl text-white">
                    {ctx.props().text.clone()}
                </div>
            </div>
        }
    }
}


