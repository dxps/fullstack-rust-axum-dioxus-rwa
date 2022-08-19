use dioxus::{events::FormData, prelude::*};

#[derive(Props)]
pub struct FormFieldProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    placeholder: Option<String>,
}

pub fn FormField_Lg<'a>(cx: Scope<'a, FormFieldProps<'a>>) -> Element {
    let ph = cx.props.placeholder.clone().unwrap_or_default();
    cx.render(rsx! {
        fieldset {
            class: "form-group",
            input {
                class: "form-control form-control-lg",
                r#type: "text",
                oninput: move |evt| cx.props.oninput.call(evt.data.as_ref().clone()),
                placeholder: "{ph}",
            }
        }
    })
}
