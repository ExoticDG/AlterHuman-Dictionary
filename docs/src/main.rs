extern crate wasm_bindgen;
extern crate yew;

mod app;
mod components;
mod pages;
mod router;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<app::App>::new().render();
}
