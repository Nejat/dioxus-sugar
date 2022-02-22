use dioxus_sugar::events;

#[events(42)]
struct PropsInteger {}

#[events("onclick")]
struct PropsString {}

fn main() {}