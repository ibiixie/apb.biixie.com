use dioxus::prelude::*;

use palette::{FromColor, Hsl, IntoColor, Lab, Laba, Srgb, color_difference::Ciede2000};

use core::convert::TryFrom;
use std::{any::Any, str::FromStr};

use crate::components::{
    comparison::ColorComparison, footer::Footer, header::Header, picker::ColorPicker,
};

#[component]
pub fn Layout() -> Element {
    let default_color: Srgb<f32> = Srgb::from_str("#ffffff").unwrap().into_format();
    let default_color: Hsl = default_color.into_color();
    let mut selected_color = use_signal(|| default_color);
    let mut computed_color = use_signal(|| default_color);
    let mut delta_e = use_signal(|| 0f32);

    rsx! {
        Header { }

        main {
            class: "container",
            section {
                id: "converter",

                article {
                    ColorPicker {
                        header: "Select a color...",
                        onchange: move |evt: Event<FormData>| {
                            selected_color.set(Srgb::from_str(&evt.value()).unwrap().into_format().into_color());

                            let mut computed = selected_color();
                            computed.hue = (computed.hue.into_positive_degrees() / 32f32).into();
                            computed.hue = computed.hue.into_positive_degrees().round().into();

                            computed.saturation /= 8f32;
                            computed.saturation = computed.saturation.round();

                            computed.lightness /= 16f32;
                            computed.lightness = computed.lightness.round();

                            computed_color.set(computed);

                            let selected_lab: Lab = selected_color().into_color();
                            let computed_lab: Lab = selected_color().into_color();

                            let delta = selected_lab.difference(computed_lab);
                            delta_e.set(delta);
                        }
                    }
                }

                article {
                    ColorComparison {
                        color_a: selected_color,
                        color_b: computed_color,
                        delta_e
                    }
                }
            }
        }
        Footer { }
    }
}
