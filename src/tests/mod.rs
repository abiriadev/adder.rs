use super::greeting::*;
use super::pipe::*;
use super::rectangle::*;
use crate::add_two;
use crate::guess::Guess;

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn another() {
    // panic!("test failed!");
    println!("don't panic.");
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        length: 8,
        width: 7,
    };

    let smaller = Rectangle {
        length: 5,
        width: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn larger_can_hold_smaller3() {
    let larger = Rectangle {
        length: 8,
        width: 7,
    };

    let smaller = Rectangle {
        length: 5,
        width: 1,
    };

    assert!(!smaller.can_hold(&larger));
}

#[test]
fn larger_can_hold_smaller2() {
    // let larger = Rectangle {
    //     length: 8,
    //     width: 7,
    // };

    let smaller = Rectangle {
        length: 5,
        width: 1,
    };

    assert!(!smaller.can_hold(&smaller));
}

#[test]
fn add_two_test() {
    assert_eq!(add_two(1), 3);
}

#[test]
fn greeting_contains_name() {
    let name = "Abiria";

    let result = greeting(name);

    assert!(
        result.contains(name),
        "RESULT OF GREETING FUNCTION DOES NOT CONTAINS NAME: {}",
        result
    );
}

#[test]
#[should_panic(expected = "less")]
fn grater_than_100() {
    Guess::new(200);
}

#[test]
fn piping() {
    let data = 123;

    assert_eq!(pipe(data), data);
}

#[test]
#[ignore]
fn ignored_test() {
    assert!(true);
}
