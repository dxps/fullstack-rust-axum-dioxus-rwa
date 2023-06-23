use dioxus::prelude::*;
use dioxus_router::Link;

use crate::commons::AppState;

pub fn Header(cx: Scope) -> Element {
    //
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    log::info!(":: Header :: app_state={:?}", app_state.read());

    let (hide_sign_in, hide_sign_out) = get_hide_sign_in_out(app_state);
    log::debug!(":: Header :: hide_sign_in={hide_sign_in} hide_sign_out={hide_sign_out}");

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
                        hidden: hide_sign_in,
                        class: "nav-item",
                        Link { class:"nav-link", to: "/signin", "Sign in" }
                    }
                    li {
                        hidden: hide_sign_out,
                        class: "nav-item",
                        Link { class:"nav-link", to: "/signout", "Sign out" }
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

fn get_hide_sign_in_out<'a>(state: &'a UseSharedState<AppState>) -> (&'a str, &'a str) {
    //
    if state.read().token.is_some() {
        ("true", "false")
    } else {
        ("false", "true")
    }
}
