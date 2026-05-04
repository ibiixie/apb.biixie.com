#![allow(clippy::volatile_composites)]

use dioxus::prelude::*;

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
