use dioxus::prelude::*;
use dioxus_router::Link;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "home-page",
            div {
                class: "banner",
                div {
                    class: "container",
                    h1 {
                        class: "logo-font", "conduit",
                        p { "A place to share your knowledge" }
                    }
                }
            }
            div {
                class: "container page",
                div {
                    class: "row",

                    div {
                        class: "col-md-9",
                        div {
                            class: "feed-toggle",
                            ul {
                                class: "nav nav-pills outline-active",
                                li {
                                    class: "nav-item",
                                    Link { class: "nav-link disabled", to: "", "Your Feed" }
                                }
                                li {
                                    class: "nav-item",
                                    Link { class: "nav-link active", to: "", "Global Feed" }
                                }
                            }
                        }

                        div {
                            class: "article-preview",
                            div {
                                class: "article-meta",
                                Link { to: "/user/tbd/profile", img { src: "http://i.imgur.com/Qr71crq.jpg" } }
                                div {
                                    class: "info",
                                    Link { to: "/user/tbd", "Eric Simons" }
                                    span { class: "date", "January 20th"}
                                }
                                button {
                                    class: "btn btn-outline-primary btn-sm pull-xs-right",
                                    i { class: "ion-heart" }
                                    " 29"
                                }
                            }
                            Link {
                                class: "preview-link", to: "/article/tbd",
                                h2 { "How to build webapps that scale" }
                                p { "This is the description of the post" }
                                span { "Read more ..." }
                            }
                        }

                        div {
                            class: "article-preview",
                            div {
                                class: "article-meta",
                                Link { to: "/user/tbd/profile", img { src: "http://i.imgur.com/N4VcUeJ.jpg" } }
                                div {
                                    class: "info",
                                    Link { to: "/user/tbd", "Albert Pai" }
                                    span { class: "date", "January 20th"}
                                }
                                button {
                                    class: "btn btn-outline-primary btn-sm pull-xs-right",
                                    i { class: "ion-heart" }
                                    " 29"
                                }
                            }
                            Link {
                                class: "preview-link", to: "/article/tbd",
                                h2 { "The song you won't ever stop singing. No matter how hard you try." }
                                p { "This is the description of the post" }
                                span { "Read more ..." }
                            }
                        }
                    }

                    div {
                        class: "col-md-3",
                        div {
                            class: "sidebar",
                            p { "Popular Tags" }
                            div {
                                class: "tag-list",
                                Link { to: "", class: "tag-pill tag-default", "programming" }
                                Link { to: "", class: "tag-pill tag-default", "javascript" }
                                Link { to: "", class: "tag-pill tag-default", "emberjs" }
                                Link { to: "", class: "tag-pill tag-default", "angularjs" }
                                Link { to: "", class: "tag-pill tag-default", "react" }
                                Link { to: "", class: "tag-pill tag-default", "mean" }
                                Link { to: "", class: "tag-pill tag-default", "node" }
                                Link { to: "", class: "tag-pill tag-default", "rust" }
                            }
                        }
                    }
                }
            }
        }
    })
}
