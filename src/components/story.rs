use dioxus::prelude::*;

pub fn Story() -> Element {
    rsx! {
        div {
            class: "w-full bg-blue-200 flex items-center justify-center",
            "This is the Story component!"
        }
    }
}
