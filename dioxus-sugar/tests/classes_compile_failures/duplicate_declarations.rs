use dioxus_sugar::classes;

#[classes(name, id)]
struct PropsMultipleDupes {
    #[class]
    name: String,
    #[class]
    id: String
}

#[classes(name)]
struct PropsSingleDupe {
    #[class]
    name: String,
    #[class]
    id: String
}

fn main() {}