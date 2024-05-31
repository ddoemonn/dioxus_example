use dioxus::prelude::*;

pub fn Ball() -> Element {
    rsx! {
        div {
            class: "w-full bg-red-200 flex items-center justify-center",
            "This is the Ball component!"
        }
    }
}
