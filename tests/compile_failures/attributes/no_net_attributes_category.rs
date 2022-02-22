use dioxus_sugar::attributes;

#[attributes(global, exclude(accesskey, class, contenteditable, data, dir, draggable, hidden, id, lang, spellcheck, style, tabindex, title, translate))]
struct Props {
    name: String,
    id: String,
}

fn main() {}