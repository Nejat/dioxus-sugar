use dioxus_sugar::attributes;

#[attributes(divv)]
struct Props {}

#[attributes(div, exclude(hrf))]
struct Props2 {}

#[attributes(div, default(hrf))]
struct Props3 {}

#[attributes(div, optional(hrf))]
struct Props4 {}

fn main() {}