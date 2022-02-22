use dioxus_sugar::events;

#[events(form_events, exclude(form_events))]
struct Props {
    name: String,
    id: String,
}

#[events(exclude(form_events))]
struct Props2 {
    name: String,
    id: String,
}

fn main() {}