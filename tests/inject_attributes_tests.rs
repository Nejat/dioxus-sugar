use dioxus::events::MouseEvent;
use dioxus::prelude::*;

use dioxus_sugar::{attributes, events};

/*
#[attributes(div, selector = "div:[1] > :any")]
#[events(keyboard, root)]
 */

#[attributes(optional(div), exclude(class, name, id))]
#[events(optional(keyboard), exclude(onkeypress))]
#[derive(Props)]
pub struct ComponentProps<'a> {
    #[props(default, add_as = "class", selector = ":root")]
    // #[props(default)]
    // #[props(default, add_as = "class", selector = "div:[2]; div:[1] > a")]
    my_class: &'a str,
    #[props(default, selector = "div")]
    // #[props(default)]
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

    // cx.render(rsx! {
    //     div {
    //         a {}
    //         &cx.props.children
    //     }
    //     div {}
    // })
}

#[attributes(optional(div), exclude(class, name, id))]
#[derive(Props)]
pub struct Component2Prop<'a> {
    // #[props(default)]
    #[props(default, add_as = "my_class", selector = "div > Component")]
    class: &'a str,
    // #[props(default)]
    #[props(default, selector = "div > Component")]
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Component2<'a>(cx: Scope<'a, Component2Prop<'a>>) -> Element {
    let _x = cx.render(rsx! { div {} });

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

/*
    cx.render(rsx! {
        div {
            ..cx.props.class,
            Component {
                &cx.props.children,
            }
            div {}
        }
        div {}
    })
*/
}

#[allow(non_snake_case)]
pub fn Component3<'a>(cx: Scope<'a>) -> Element {
    rsx! {
        cx,
        div {
            div {}
        }
    }
}