use dioxus::{events::FormData, prelude::*};

#[derive(Props)]
pub struct FormTextareaProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    rows: Option<u8>,

    #[props(optional)]
    placeholder: Option<String>,
}

pub fn FormTextarea_Lg<'a>(cx: Scope<'a, FormTextareaProps<'a>>) -> Element {
    let rows = cx.props.rows.unwrap_or(1);
    let ph = cx.props.placeholder.clone().unwrap_or_default();
    cx.render(rsx! {
        fieldset {
            class: "form-group",
            textarea {
                class: "form-control form-control-lg",
                oninput: move |evt| cx.props.oninput.call(evt.data.as_ref().clone()),
                placeholder: "{ph}",
                rows: "{rows}"
            }
        }
    })
}
