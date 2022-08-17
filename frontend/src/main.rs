#![allow(non_snake_case)]

mod comps;
mod pages;

use crate::comps::{Footer, NavBar};
use crate::pages::HomePage;
use dioxus::prelude::*;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(Router {
         NavBar { }
         Route { to: "/", HomePage { }}
         Footer{ }
    }))
}
