#![allow(non_snake_case)]
use dioxus::prelude::*;
mod components;
use components::ball::Ball;
use components::story::Story;

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx! {
        Story {},
        Ball {}

    }
}
