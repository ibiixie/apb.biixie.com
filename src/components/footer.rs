use dioxus::prelude::*;

static HEART_ICON: Asset = asset!("/assets/rainbow-heart.svg");

#[component]
pub fn Footer() -> Element {
    let commit_hash = env!("GIT_COMMIT_HASH");
    let commit_short = env!("GIT_COMMIT_SHORT");
    let commit_url = format!("https://codeberg.org/biixie/apb.biixie.com/commit/{commit_hash}");

    rsx! {
        footer {
            id: "page-footer",
            class: "container",

            div {
                small {
                    span {
                        "Hastily undercooked with "
                    }
                    span {
                        "data-tooltip": ":3",

                        img {
                            src: HEART_ICON,
                            width: "24px",
                            height: "24px",
                        }
                    }
                    " by "
                    a {
                        href: "https://biixie.com",
                        "Biixie"
                    }
                    "~"
                }
                br {}
                small {
                    a {
                        href: "https://codeberg.org/biixie/apb.biixie.com",
                        "Source code"
                    }
                    " dual-licensed under either "
                    a {
                        href: "https://codeberg.org/biixie/apb.biixie.com/src/branch/main/LICENSE-MIT",
                        "MIT"
                    }
                    " or "
                    a {
                        href: "https://codeberg.org/biixie/apb.biixie.com/src/branch/main/LICENSE-APACHE",
                        "Apache 2.0"
                    }
                }
                br {}
                small {
                    "Website version "
                    a {
                        href: commit_url,
                        {commit_short}
                    }
                }
            }
        }

    }
}
