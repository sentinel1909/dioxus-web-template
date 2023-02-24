// main.rs

#![allow(non_snake_case)]
use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}

fn main() {
    dioxus_web::launch(App);
}
