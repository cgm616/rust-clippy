// run-rustfix
#![warn(clippy::xor_used_as_pow)]
#![allow(clippy::identity_op)]

// Should not be linted
#[allow(dead_code)]
enum E {
    First = 1 ^ 8,
    Second = 2 ^ 8,
    Third = 3 ^ 8,
    Tenth = 10 ^ 8,
}

fn main() {
    // These should succeed:
    let _ = 9 ^ 3; // lhs other than 2 or 10
    let _ = 0x02 ^ 6; // lhs not decimal
    let _ = 2 ^ 0x10; // rhs hexadecimal
    let _ = 10 ^ 0b0101; // rhs binary
    let _ = 2 ^ 0o1; // rhs octal
    let _ = 10 ^ -18; // negative rhs
    let _ = 2 ^ 0; // zero rhs

    // These should fail
    let _ = 2 ^ 3;
    let _ = 2 ^ 32;
    {
        let x = 15;
        let _ = 2 ^ x;
    }
}
