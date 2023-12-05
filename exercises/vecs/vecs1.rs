// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// so [i32; 4] is type; length array type
// and Vec is a vector whose length can change. I guess it's like Python's list
fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array

    // This is one way of doing it, but we need to make the vector mutable to
    // change its contents
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    // This way is more succinct
    let v = vec![10, 20, 30, 40];

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
