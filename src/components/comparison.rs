use dioxus::prelude::*;

use dioxus::logger::tracing;

use palette::Hsl;

#[component]
pub fn ColorComparison(
    color_a: ReadSignal<Hsl>,
    color_b: ReadSignal<Hsl>,
    delta_e: Signal<f32>,
) -> Element {
    tracing::info!(
        "color_a: {color_a:?},
        color_b: {color_b:?},
        delta_e: {delta_e}"
    );

    rsx! {
        div {
            class: "result",
            header {
                "Result & Comparison (",
                span {
                    "data-tooltip": "A ΔE value of < 1.00 is considered an imperceptible difference to most humans.",

                    "ΔE2000 = ",

                    span {
                        id: "cmp-result-delta-e",
                        "{delta_e}"
                    }
                },
                ")"

                sup {
                    a {
                        href: "#footer-anchor-1",
                        "1"
                    }
                }
            }

            div {
                class: "grid comparison",
                article {
                    header {
                        "Original"
                    }
                    div {
                        id: "cmp-original"
                    }

                    footer {
                        small {
                            span {
                                id: "cmp-original-info-hex",
                                "N/A"
                            }
                            br {}
                            span {
                                id: "cmp-original-info-rgb",
                                "N/A"
                            }
                            br {}
                            span {
                                id: "cmp-original-info-hsl",
                                "N/A"
                            }
                        }
                    }
                }

                article {
                    header {
                        "Result"
                    }
                    div {
                        id: "cmp-result"
                    }
                    footer {
                        small {
                            span {
                                id: "cmp-result-info-hex",
                                "N/A"
                            }
                            br {}
                            span {
                                id: "cmp-result-info-rgb",
                                "N/A"
                            }
                            br {}
                            span {
                                id: "cmp-result-info-hsl",
                                "N/A"
                            }
                            br {}
                            span {
                                id: "cmp-result-info-apb",
                                "N/A"
                            }
                        }
                    }
                }
            }
        }
    }
}
