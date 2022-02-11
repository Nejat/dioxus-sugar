use trybuild::TestCases;

use dioxus_sugar::classes;

#[test]
fn expected_classes_compile_failures() {
    let tests = TestCases::new();

    tests.compile_fail("tests/classes_compile_failures/*.rs");
}

#[test]
fn given_a_struct_with_fields_listed_in_classes_attr_should_display_fields() {
    #[classes(id)]
    struct Sut {
        _name: &'static str,
        id: &'static str,
    }

    let sut = Sut { _name: "Jack", id: "jk1" };

    assert_eq!("jk1", format!("{}", sut));
}

#[test]
fn given_a_struct_with_fields_decorated_with_class_attr_should_display_fields() {
    #[classes]
    struct Sut {
        _name: &'static str,
        #[class]
        id: &'static str,
    }

    let sut = Sut { _name: "Jack", id: "jk1" };

    assert_eq!("jk1", format!("{}", sut));
}

#[test]
fn given_a_struct_with_class_fields_and_classes_attr_should_display_fields() {
    #[classes(name)]
    struct Sut {
        name: &'static str,
        #[class]
        id: &'static str,
    }

    let sut = Sut { name: "Jack", id: "jk1" };

    assert_eq!("Jackjk1", format!("{}", sut));
}
