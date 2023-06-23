use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};

use crate::comps::{FormButton_Lg, FormInput_Lg, FormTextarea_Lg};

pub fn SettingsPage(cx: Scope) -> Element {
    let profilePictureURL = use_state(&cx, String::new);
    let name = use_state(&cx, String::new);
    let biography = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    cx.render(rsx! {
        div {
            class: "settings-page",
            div {
                class: "container page",
                div {
                    class: "col-md-6 offset-md-3 col-xs-12",
                    h1 {
                        class: "text-xs-center", "Your Settings"
                    }
                    br {}
                    br {}

                    form {
                        FormInput_Lg{
                            oninput: move |s: FormData| profilePictureURL.set(s.value),
                            placeholder: "URL of profile picture".to_string()
                        }
                        FormInput_Lg {
                            oninput: move |s: FormData| name.set(s.value),
                            placeholder: "Your Name".to_string()
                        }
                        FormTextarea_Lg{
                            oninput: move |s: FormData| biography.set(s.value),
                            placeholder: "Short bio about you".to_string()
                        }
                        FormInput_Lg {
                            oninput: move |s: FormData| email.set(s.value),
                            placeholder: "Email".to_string()
                        }
                        FormInput_Lg {
                            oninput: move |s: FormData| password.set(s.value),
                            placeholder: "Password".to_string()
                        }
                        FormButton_Lg {
                            onclick: move |_: MouseEvent| {
                                log::info!(":: SettingsPage] button clicked. name: {} | email: {}", name, email);
                                // TODO: Call the corresponding (HTTP) API operation, and all the rest. 
                            },
                            label: "Update Settings".to_string()
                        }
                    }
                }
            }
        }
    })
}
