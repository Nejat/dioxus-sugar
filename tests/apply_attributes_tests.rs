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
            &cx.props.children
        }
    })
}

// // intended result
// fn component2<'a>(cx: Scope<'a, Props<'a>>) -> Element {
//     cx.render(rsx! {
//         div {
//             class : "{cx.props.class}",
//             accesskey : "{cx.props.accesskey:?}", contenteditable :
//             "{cx.props.contenteditable:?}", dir : "{cx.props.dir:?}", draggable :
//             "{cx.props.draggable:?}", hidden : "{cx.props.hidden:?}", lang :
//             "{cx.props.lang:?}", spellcheck : "{cx.props.spellcheck:?}", style :
//             "{cx.props.style:?}", tabindex : "{cx.props.tabindex:?}", title :
//             "{cx.props.title:?}", translate : "{cx.props.translate:?}",
//             &cx.props.children
//         }
//     })
// }
