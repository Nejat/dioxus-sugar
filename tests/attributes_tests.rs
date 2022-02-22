use dioxus_sugar::attributes;

use trybuild::TestCases;

#[test]
fn expected_attributes_compile_failures() {
    let tests = TestCases::new();

    tests.compile_fail("tests/compile_failures/attributes/*.rs");
}

#[test]
fn given_a_struct_with_attribute_extensions_it_should_extend_struct() {
    #[attributes(href, hidden, disabled)]
    struct _Sut;
}

#[test]
fn given_a_struct_with_attribute_and_tag_extensions_it_should_extend_struct() {
    #[attributes(href, hidden, disabled, div)]
    struct _Sut;
}

#[test]
fn given_a_struct_with_attribute_and_tag_extensions_and_excludes_listed_it_should_extend_struct_within_limits() {
    #[attributes(href, hidden, disabled, div, exclude(id, class, style))]
    struct _Sut;
}

#[test]
fn given_a_struct_with_tag_extensions_it_should_extend_struct() {
    #[attributes(button, div, basic)]
    struct _Sut;
}
