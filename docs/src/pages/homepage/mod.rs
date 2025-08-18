use yew::prelude::*;

use crate::components::HeroImage;
pub enum Msg {}

pub struct Homepage {}

impl Component for Homepage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col flex-1">
                <HeroImage
                    text={"Protect the puppy!"}
                />
                <div class="bg-slate-500 flex flex-row flex-wrap gap-12">
                    <div class="bg-contain bg-no-repeat bg-center w-1/3">
                        <img class="w-fit" src="images/ImageGallery/image_1.jpeg"/>
                    </div>
                    <div class="bg-contain bg-no-repeat bg-center w-1/3">
                        <img class="w-fit" src="images/ImageGallery/image_2.jpeg"/>
                    </div>
                    <div class="bg-contain bg-no-repeat bg-center w-1/3">
                        <img class="w-fit" src="images/ImageGallery/image_3.jpeg"/>
                    </div>
                    <div class="bg-contain bg-no-repeat bg-center w-1/3">
                        <img class="w-fit" src="images/ImageGallery/image_4.jpeg"/>
                    </div>
                </div>
            </div>
        }
    }
}
