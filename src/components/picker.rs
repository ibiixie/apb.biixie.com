use dioxus::prelude::*;

#[component]
pub fn ColorPicker(onchange: EventHandler<FormEvent>, header: Option<String>) -> Element {
    rsx! {
        div {
            class: "picker",
            {
                header.map_or_else(|| { rsx! {} }, |v| rsx! { header { "{v}" } })
            }
            input {
                type: "color",
                id: "color-input",
                name: "color",
                value: "#ffffff",
                onchange
            }
        }
    }
}
