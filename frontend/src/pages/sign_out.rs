use dioxus::prelude::*;
use dioxus_router::use_router;
use dioxus_use_storage::use_session_storage;

use crate::commons::TOKEN;

pub fn SignOutPage(cx: Scope) -> Element {
    //
    use_session_storage(cx).remove(TOKEN);
    use_router(&cx).push_route("/home", None, None);

    cx.render(rsx! {
        div {
            "Signed out"
        }
    })
}
