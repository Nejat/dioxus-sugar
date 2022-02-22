use dioxus_sugar::attributes;

#[attributes(href, div, exclude(href, div, class))]
struct Props {
    name: String,
    id: String,
}

#[attributes(exclude(href, div, class))]
struct Props2 {
    name: String,
    id: String,
}

fn main() {}