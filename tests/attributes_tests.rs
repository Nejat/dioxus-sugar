use syn::__private::bool;
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
        href: String::from("https://duckduckgo.com"),
        hidden: String::from("false"),
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
    struct Sut {
        group: String,
        color: String
    }

    let sut = Sut {
        group: String::from("Leader"),
        color: String::from("red"),
        href: String::from("https://duckduckgo.com"),
        hidden: String::from("false"),
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
    struct _Sut {
        group: String
    }
}

#[test]
fn given_a_struct_with_attribute_and_tag_extensions_and_exclude_listed_it_should_extend_struct_within_limits() {
    #[attributes(href, disabled, div, exclude(id, class, style))]
    struct Sut {
        group: String
    }

    let _sut = Sut {
        group: String::from("Leader"),
        draggable: String::default(),
        lang: String::default(),
        href: String::default(),
        spellcheck: String::default(),
        title: String::default(),
        tabindex: String::default(),
        translate: String::default(),
        hidden: String::default(),
        dir: String::default(),
        contenteditable: String::default(),
        accesskey: String::default(),
        disabled: bool::default(),
    };

    // #[attributes(href, disabled, div)]
    struct _Sut2 {
        group: String
    }
}

#[test]
fn given_a_struct_with_tag_extensions_it_should_extend_struct() {
    #[attributes(button, div, basic)]
    struct _Sut {
        group: String
    }
}
