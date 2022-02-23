use trybuild::TestCases;

use dioxus_sugar::events;

#[test]
fn expected_events_compile_failures() {
    let tests = TestCases::new();

    tests.compile_fail("tests/compile_failures/events/*.rs");
}

#[test]
fn given_a_struct_with_event_extensions_it_should_extend_struct() {
    #[events(onclick, onkeypress)]
    struct _Sut<'a>;
}

#[test]
fn given_a_struct_with_event_extensions_and_excludes_listed_it_should_extend_struct_within_limits() {
    #[events(keyboard, exclude(onkeypress))]
    struct _Sut<'a>;
}
