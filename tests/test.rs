#[macro_use]
extern crate serde_cast;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct A {
    a: String,
    b: (u32, u32),
}

#[derive(Serialize, Deserialize)]
struct B {
    a: String,
    b: (u32, u32),
}

#[derive(Serialize, Deserialize)]
struct C {
    a: i32,
    b: (u32, u32),
}


#[test]
fn valid_cast() {
    let a = A {
        a: "Hello, world!".to_owned(),
        b: (2, 3),
    };
    let b: B = serde_cast!(&a);

    assert_eq!(a.a, b.a);
    assert_eq!(a.b, b.b);
}

#[test]
#[should_panic(expected = "serde_cast failed")]
fn invalid_cast() {
    let a = A {
        a: "Hello, world!".to_owned(),
        b: (2, 3),
    };
    let c: C = serde_cast!(&a);

    let _ = c;
}

#[test]
fn the_only_example_code() {
    use serde::{Serialize, Deserialize};
    use serde_cast::serde_cast;

    #[derive(Serialize)]
    struct A {
        field: String,
    }

    #[derive(Deserialize)]
    struct B {
        field: String,
    }

    let a = A { field: "This was a mistake".to_string() };
    let b: B = serde_cast!(&a);

    assert_eq!(a.field, b.field);
}
