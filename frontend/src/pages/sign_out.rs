use dioxus::prelude::*;
use dioxus_router::use_router;
use dioxus_use_storage::use_session_storage;

use crate::commons::{AppState, TOKEN};

pub fn SignOutPage(cx: Scope) -> Element {
    //
    use_session_storage(&cx).remove(TOKEN);
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    app_state.write().token = None;
    use_router(&cx).push_route("/home", None, None);

    cx.render(rsx! {
        div {
            h2 { "Signed out" }
        }
    })
}
