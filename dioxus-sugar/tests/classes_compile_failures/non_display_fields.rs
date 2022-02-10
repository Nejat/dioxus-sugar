use dioxus_sugar::classes;

enum Value {
    One,
    Two,
    Three,
}

#[classes(value)]
struct Props {
    value: Value,
}

fn main() {}