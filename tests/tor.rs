#![feature(phase)]

#[phase(plugin)]
extern crate tor;

#[test]
fn parse_errors() {
    //tor![0u8; 1, 2];
    //~^ error: expected `]`, found `;`

    //tor![0u8 1, 2];
    //~^ error: expected `]`, found `1`

    //tor![0u8, ..];
    //~^ error: unexpected token: `]`
}

#[test]
fn plain() {
    assert_eq!(vec![0u8, 1, 2], tor![0u8, 1, 2])
}

#[test]
fn value_repeat() {
    assert_eq!(vec![0u8, 0, 0, 0], tor![0u8, ..4])
}
