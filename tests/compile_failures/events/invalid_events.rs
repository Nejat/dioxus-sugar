use dioxus_sugar::events;

#[events(onclik)]
struct Props {}

#[events(keyboard, exclude(onkyprss))]
struct Props2 {}

fn main() {}