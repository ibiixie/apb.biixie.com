use dioxus::prelude::*;

use crate::color::ColorWrapper;

#[component]
pub fn ComparisonApplet(color: ReadSignal<Option<ColorWrapper>>) -> Element {
    let color = color.read().cloned().unwrap_or_default();
    let color_hsl = color.inner();
    let color_converted = ColorWrapper::from_apb(color.to_apb());

    rsx! {
        div {
            class: "comparison grid",
            article {
                header { h6 { "Original" } }
                div {
                    background_color: "{color.to_string()}",
                    class: "before",
                }
                footer {
                    dl {
                        class: "details",
                        dl {
                            "HEX: {color.to_string()}"
                        }
                        dl {
                            "HSL: {color_hsl.hue.into_positive_degrees():.0} {color_hsl.saturation:.2} {color_hsl.lightness:.2}"
                        }
                    }
                }
            }
            article {
                header { h6 { "Converted" } }
                div {
                    background_color: "{color_converted.to_string()}",
                    class: "after",
                }
                footer {
                    dl {
                        class: "details",
                        dl {
                            "HEX: {color_converted.to_string()}"
                        }
                        dl {
                            "HSL: {color_converted.inner().hue.into_positive_degrees():.0} {color_converted.inner().saturation:.2} {color_converted.inner().lightness:.2}"
                        }
                        dl {
                            "APB: {color_converted.to_apb().hue} {color_converted.to_apb().saturation} {color_converted.to_apb().lightness}"
                        }
                    }
                }
            }
        }
    }
}
