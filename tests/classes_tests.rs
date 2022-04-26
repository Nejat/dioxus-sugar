use trybuild::TestCases;

use dioxus_sugar::classes;

#[test]
fn expected_classes_compile_failures() {
    let tests = TestCases::new();

    tests.compile_fail("tests/compile_failures/classes/*.rs");
}

#[test]
fn given_a_struct_with_fields_listed_in_classes_attr_should_display_fields() {
    #[classes(other_field)]
    struct Sut {
        _field_one: &'static str,
        other_field: &'static str,
    }

    let sut = Sut { _field_one: "Jack", other_field: "jk1" };

    assert_eq!("jk1", format!("{}", sut));
}

#[test]
fn given_a_struct_with_fields_decorated_with_class_attr_should_display_fields() {
    #[classes]
    struct Sut {
        _field_one: &'static str,
        #[class]
        other_field: &'static str,
    }

    let sut = Sut { _field_one: "Jack", other_field: "jk1" };

    assert_eq!("jk1", format!("{}", sut));
}

#[test]
fn given_a_struct_with_class_fields_and_classes_attr_should_display_fields() {
    #[classes(field_one)]
    struct Sut {
        field_one: &'static str,
        #[class]
        other_field: &'static str,
    }

    let sut = Sut { field_one: "Jack", other_field: "jk1" };

    assert_eq!("Jack jk1", format!("{}", sut));
}
