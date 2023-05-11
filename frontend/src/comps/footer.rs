use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            div {
                class:"container",
                Link { class:"logo-font", to:"/", "conduit" }
                span {
                    class:"attribution",
                    "An interactive learning project from ",
                    a {
                        href:"https://thinkster.io", target: "_blank",
                        "Thinkster"
                    }
                    ". Code & design licensed under MIT."
                }
            }
        }
    })
}
