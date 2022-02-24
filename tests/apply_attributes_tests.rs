use dioxus::prelude::*;

use dioxus_sugar::{apply_attributes, attributes};

#[attributes(optional(div), exclude(class, name, id))]
#[derive(Props)]
struct Props<'a> {
    #[props(default)]
    class: &'a str,
    children: Element<'a>,
}

fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        div {
            apply_attributes!{ cx; div, exclude(class, name, id) }
            &cx.props.children
        }
    })
}