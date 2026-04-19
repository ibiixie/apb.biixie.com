use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            id: "page-header",
            class: "container",
            hgroup {
                h2 {
                    "APB "
                    span { color: "#E40303", "C" }
                    span { color: "#FF8C00", "o" }
                    span { color: "#FFED00", "l" }
                    span { color: "#008026", "o" }
                    span { color: "#004CFF", "r" }
                    " Converter"
                }
                p { "Convert any color into the closest equivalent in the APB: Reloaded palette." }
            }
        }
    }
}
