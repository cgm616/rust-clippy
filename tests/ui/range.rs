#[warn(clippy::range_zip_with_len)]
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let _x = v1.iter().zip(0..v1.len());
    let _y = v1.iter().zip(0..v2.len()); // No error
}

#[allow(unused)]
fn no_panic_with_fake_range_types() {
    struct Range {
        foo: i32,
    }

    let _ = Range { foo: 0 };
}

#[warn(clippy::manual_range_contains)]
#[allow(unused)]
#[allow(clippy::no_effect)]
#[allow(clippy::short_circuit_statement)]
fn manual_range_contains(x: u32) {
    // order shouldn't matter
    x >= 8 && x < 12;
    x < 42 && x >= 21;
    100 > x && 1 <= x;

    // also with inclusive ranges
    x >= 9 && x <= 99;
    x <= 33 && x >= 1;
    999 >= x && 1 <= x;

    // and the outside
    x < 8 || x >= 12;
    x >= 42 || x < 21;
    100 <= x || 1 > x;

    // also with the outside of inclusive ranges
    x < 9 || x > 99;
    x > 33 || x < 1;
    999 < x || 1 > x;

    // not a range.contains
    x > 8 && x < 12; // lower bound not inclusive
    x < 8 && x <= 12; // same direction
    x >= 12 && 12 >= x; // same bounds
    x < 8 && x > 12; // wrong direction

    x <= 8 || x >= 12;
    x >= 8 || x >= 12;
    x < 12 || 12 < x;
    x >= 8 || x <= 12;
}
