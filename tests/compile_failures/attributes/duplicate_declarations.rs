use dioxus_sugar::attributes;

#[attributes(href, hidden, disabled)]
struct Sut {
    hidden: bool,
}

fn main() {
    let _sut = Sut {
        href: String::from("https://duckduckgo.com"),
        hidden: String::from("false"),
        disabled: false,
    };
}
