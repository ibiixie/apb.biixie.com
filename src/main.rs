#![allow(clippy::volatile_composites)]

use dioxus::prelude::*;

use dioxus::logger::tracing;

mod components;
use components::comparison_applet::ComparisonApplet;
use components::footer::Footer;
use components::header::Header;

mod color;
use color::ColorWrapper;

static STYLE: Asset = asset!("/assets/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut color = use_signal(|| None::<ColorWrapper>);

    let original = ColorWrapper::from_apb(color::ApbColor {
        hue: 18,
        saturation: 7,
        lightness: 3,
    });

    // let original = ColorWrapper::from_hex("#115778");

    tracing::info!(
        "After from_apb:  H={:.6} S={:.6} L={:.6}",
        original.inner().hue.into_positive_degrees(),
        original.inner().saturation,
        original.inner().lightness
    );

    let hex = original.to_hex();
    tracing::info!("Hex: {}", hex);

    let roundtripped = ColorWrapper::from_hex(&hex);
    tracing::info!(
        "After roundtrip: H={:.6} S={:.6} L={:.6}",
        roundtripped.inner().hue.into_positive_degrees(),
        roundtripped.inner().saturation,
        roundtripped.inner().lightness
    );

    let apb = roundtripped.to_apb();
    tracing::info!(
        "to_apb result: H={} S={} L={}",
        apb.hue,
        apb.saturation,
        apb.lightness
    );

    rsx! {
        title { "APB Color Converter" }

        meta {
            name: "color-scheme",
            content: "light dark"
        }

        meta {
            name: "description",
            content: "Convert any color to its APB: Reloaded counterpart."
        }

        Stylesheet {
            href: "https://cdn.jsdelivr.net/npm/@picocss/pico@2.1.1/css/pico.fuchsia.min.css"
        }

        Stylesheet {
            href: "https://cdn.jsdelivr.net/npm/@picocss/pico@2.1.1/css/pico.colors.min.css"
        }

        Stylesheet {
            href: STYLE
        }

        Header { }

        main {
            class: "container",

            section {
                article {
                    header {
                        "Select a color"
                    }
                    input {
                        type: "color",
                        value: "#00000000",
                        oninput: move |evt| {
                            color.set(Some(
                                ColorWrapper::from_hex(&evt.value())
                            ));
                        }
                    }
                    footer {
                        {
                            color.read().as_ref().map_or_else(
                                ||      rsx! { small { class: "pico-color-yellow-200", "No color selected" } },
                                |color| rsx! { small { "{color.to_string()}" } }
                            )
                        }
                    }
                }

                article {
                    header {
                        "Result ("
                        span {
                            "data-tooltip": "A value of < 1.0 is a practically imperceptible difference.",
                            {
                                let delta = color.read().as_ref().map_or_else(
                                    || "N/A".to_owned(),
                                    |color| {
                                        format!("{:.2}", color.difference(&ColorWrapper::from_apb(color.to_apb())))
                                    }
                                );

                                rsx! { "CIE ∆E2000 ≈ {delta}" }
                            }
                        }
                        ")"
                        sup {
                            a {
                                href: "#footnote-comparison-1",
                                "1"
                            }
                        }
                    }

                    ComparisonApplet { color }

                    footer {
                        small {
                            sup { "1" }
                            i {
                                a {
                                    id: "footnote-comparison-1",
                                    href: "https://muldoon.design/playground/whats-my-delta-e",
                                    "What's your ∆E?"
                                }
                            }
                        }
                    }
                }

            }
        }

        Footer { }
    }
}
