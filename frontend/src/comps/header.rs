use dioxus::prelude::*;
use dioxus_router::Link;

use crate::commons::AppState;

pub fn Header(cx: Scope) -> Element {
    //
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    log::info!(":: Header :: app_state={:?}", app_state.read());

    let (signed_in, signed_out) = is_signed_in_or_not(app_state);
    log::debug!(":: Header :: signed_in={signed_in} signed_out={signed_out}");

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
                        Link { class:"nav-link", to: "/", "Home" }
                    }
                    li {
                        hidden: signed_out,
                        class:"nav-item",
                        Link {
                            class: "nav-link", to: "/article_add",
                            i { class: "ion-compose" },
                            " New Article",
                        }
                    }
                    li {
                        hidden: signed_out,
                        class:"nav-item",
                        Link {
                            class: "nav-link", to: "/settings",
                            i { class: "ion-gear-a" },
                            " Settings"
                        }
                    }
                    li {
                        hidden: signed_in,
                        class: "nav-item",
                        Link { class:"nav-link", to: "/signin", "Sign in" }
                    }
                    li {
                        hidden: signed_out,
                        class: "nav-item",
                        Link { class:"nav-link", to: "/signout", "Sign out" }
                    }
                    li {
                        hidden: signed_in,
                        class:"nav-item",
                        Link { class:"nav-link", to: "/signup", "Sign up" }
                    }
                }
            }
        }
    })
}

fn is_signed_in_or_not<'a>(state: &'a UseSharedState<AppState>) -> (&'a str, &'a str) {
    //
    if state.read().token.is_some() {
        ("true", "false")
    } else {
        ("false", "true")
    }
}
