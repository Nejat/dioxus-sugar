use dioxus::prelude::*;

use dioxus_sugar::attributes;

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
            &cx.props.children
        }
    })
}
