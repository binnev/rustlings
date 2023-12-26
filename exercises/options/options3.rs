// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // When we match `Some(p)`, the match statement takes ownership of p,
        // which makes it unavailable to use below as the return value. Instead
        // the match statement needs to _borrow_ p.

        // Some(&p) denotes that we expect a reference to an object.
        // Some(ref p) denotes that we want a reference to an unpacked value.
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
