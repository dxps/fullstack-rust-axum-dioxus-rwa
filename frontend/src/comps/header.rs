use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class:"navbar navbar-light",
            div {
                class: "container",
                Link { class:"navbar-brand", to: "/", "conduit" }
                ul {
                    class:"nav navbar-nav pull-xs-right",
                    li {
                        class:"nav-item",
                        // todo: Add "active" class when you're on that page.
                        Link { class:"nav-link active", to: "/", "Home" }
                    }
                    li {
                        class:"nav-item",
                        Link {
                            class: "nav-link", to: "/article_add",
                            i { class: "ion-compose" },
                            " New Article",
                        }
                    }
                    li {
                        class:"nav-item",
                        Link {
                            class: "nav-link", to: "/settings",
                            i { class: "ion-gear-a" },
                            " Settings"
                        }
                    }
                    li {
                        class:"nav-item",
                        Link { class:"nav-link", to: "/signin", "Sign in" }
                    }
                    li {
                        class:"nav-item",
                        Link { class:"nav-link", to: "/signup", "Sign up" }
                    }
                }
            }
        }
    })
}
