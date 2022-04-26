use dioxus::events::MouseEvent;
use dioxus::prelude::*;

use dioxus_sugar::{attributes, events};

#[attributes(optional(div), exclude(class, name, id))]
#[events(optional(keyboard), exclude(onkeypress))]
#[derive(Props)]
pub struct ComponentProps<'a> {
    #[props(default, inject_as = "class", selector = "div:[1]; div:[0] > a")]
    my_class: &'a str,
    #[props(default, selector = "div")]
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Component<'a>(cx: Scope<'a, ComponentProps<'a>>) -> Element {
    rsx! {
        cx: ComponentProps;
        div {
            a {}
            &cx.props.children
        }
        div {}
    }
}

#[attributes(optional(div), exclude(class, name, id))]
#[derive(Props)]
pub struct Component2Prop<'a> {
    #[props(default, inject_as = "my_class", selector = "div > Component")]
    class: &'a str,
    #[props(default, selector = "div > Component")]
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Component2<'a>(cx: Scope<'a, Component2Prop<'a>>) -> Element {
    rsx! {
        cx: Component2Prop;
        div {
            Component {
                &cx.props.children,
            }
            div {}
        }
        div {}
    }
}
