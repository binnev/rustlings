// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    // normally, multiple mutable borrows would not be allowed. But if we use
    // scopes, the first mutable borrow goes out of scope before we make the
    // second one.
    {
        let y = &mut x;
        *y += 100;
    }
    {
        let z = &mut x;
        *z += 1000;
    }
    assert_eq!(x, 1200);
}
