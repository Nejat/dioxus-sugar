use dioxus::prelude::*;
use trybuild::TestCases;

use dioxus_sugar::attributes;

#[test]
fn expected_attributes_compile_failures() {
    let tests = TestCases::new();

    tests.compile_fail("tests/compile_failures/attributes/*.rs");
}

#[test]
fn given_a_struct_with_attribute_extensions_it_should_extend_struct() {
    #[attributes(href, hidden, disabled)]
    struct Sut;

    let sut = Sut {
        href: "https://duckduckgo.com",
        hidden: "false",
        disabled: false,
    };

    assert_eq!("https://duckduckgo.com", sut.href);
    assert_eq!("false", sut.hidden);
    assert!(!sut.disabled);
}

#[test]
fn given_a_struct_with_attribute_extensions_and_existing_fields_it_should_extend_struct() {
    #[attributes(href, hidden, disabled)]
    struct SutTrailingComma {
        group: String,
        color: String,
    }

    #[attributes(href, hidden, disabled)]
    #[rustfmt::skip]
    struct Sut {
        group: String,
        // @formatter:off
        color: String
        // @formatter:on
    }

    let sut = Sut {
        group: String::from("Leader"),
        color: String::from("red"),
        href: "https://duckduckgo.com",
        hidden: "false",
        disabled: false,
    };

    assert_eq!("Leader", sut.group);
    assert_eq!("red", sut.color);
    assert_eq!("https://duckduckgo.com", sut.href);
    assert_eq!("false", sut.hidden);
    assert!(!sut.disabled);
}

#[test]
fn given_a_struct_with_attribute_and_tag_extensions_it_should_extend_struct() {
    #[attributes(href, hidden, disabled, div)]
    struct Sut {
        _group: String,
    }

    let _sut = Sut {
        _group: String::from("Leader"),
        id: "",
        class: "",
        style: "",
        draggable: "",
        lang: "",
        href: "",
        spellcheck: "",
        title: "",
        tabindex: "",
        translate: "",
        hidden: "",
        dir: "",
        contenteditable: "",
        accesskey: "",
        disabled: bool::default(),
    };
}

#[test]
fn given_a_struct_with_attribute_and_tag_extensions_and_exclude_listed_it_should_extend_struct_within_limits() {
    #[attributes(href, disabled, div, exclude(id, class, style))]
    struct Sut {
        _group: String,
    }

    let _sut = Sut {
        _group: String::from("Leader"),
        draggable: "",
        lang: "",
        href: "",
        spellcheck: "",
        title: "",
        tabindex: "",
        translate: "",
        hidden: "",
        dir: "",
        contenteditable: "",
        accesskey: "",
        disabled: bool::default(),
    };
}

#[test]
fn given_a_struct_with_default_attribute_extensions_it_should_extend_struct() {
    #[attributes(default(href, hidden, disabled = true))]
    #[derive(Props, Eq, PartialEq)]
    struct Sut;

    let sut = Sut {
        href: "https://duckduckgo.com",
        hidden: "false",
        disabled: false,
    };

    assert_eq!("https://duckduckgo.com", sut.href);
    assert_eq!("false", sut.hidden);
    assert!(!sut.disabled);
}

#[test]
fn given_a_struct_with_optional_attribute_extensions_it_should_extend_struct() {
    #[attributes(optional(href, hidden, disabled))]
    #[derive(Props, Eq, PartialEq)]
    struct Sut;

    let sut = Sut {
        href: Some("https://duckduckgo.com"),
        hidden: Some("false"),
        disabled: Some(false),
    };

    assert_eq!("https://duckduckgo.com", sut.href.unwrap());
    assert_eq!("false", sut.hidden.unwrap());
    assert!(!sut.disabled.unwrap());
}

// todo: bool, enum & number support instead of str
// todo: revaluate basic group .. xmlns?

#[test]
fn given_a_struct_with_tag_extensions_it_should_extend_struct() {
    #[attributes(button, div, /*basic*/)]
    struct Sut {
        _group: String,
    }

    let _sut = Sut {
        _group: String::from("Leader"),
        accesskey: "",
        class: "",
        contenteditable: "true",
        dir: "",
        draggable: "true",
        hidden: "false",
        id: "bar",
        lang: "en",
        spellcheck: "false",
        style: "",
        tabindex: "3",
        title: "",
        translate: "",
        // xmlns: ""
    };
}
