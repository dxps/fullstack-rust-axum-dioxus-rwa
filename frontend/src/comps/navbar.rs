use dioxus::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class:"navbar navbar-light",
            div {
                class: "container",
                a { class:"navbar-brand", href: "/", "conduit" }
                ul {
                    class:"nav navbar-nav pull-xs-right",
                    li {
                        class:"nav-item",
                        // Add "active" class when you're on that page.
                        Link { class:"nav-link active", to: "/", "Home" }
                    }
                    li {
                        class:"nav-item",
                        a { class:"nav-link", href:"#",
                            i { class:"ion-compose"},
                            " New Article"
                        }
                    }
                    li {
                        class:"nav-item",
                        a { class:"nav-link", href:"#",
                            i { class:"ion-gear-a"},
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
