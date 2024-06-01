use sycamore::prelude::*;
use web_sys::{wasm_bindgen::prelude::*, window};

use routing::*;

mod api;
mod components;
mod routing;

#[component]
pub fn App<G: Html>() -> View<G> {
    window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .set_title("ebobo.dev");

    view! {
        Root()
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}
