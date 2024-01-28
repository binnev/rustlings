// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// Obtain the number of bytes (not characters) in the given argument.
fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    // `as_ref()` auto-dereferences
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    arg.as_ref().chars().count()
}

// Squares a number using as_mut(). NOTE: we want to square the value IN-PLACE.
// We receive a mutable borrow of type T. We need to update the contained value
// without taking __ownership__ of the value.
fn num_sq<T>(arg: &mut T)
where
    T: AsMut<u32>,
{
    // I'm really struggling to figure out what is going on here. I need to
    // deref the left hand side, but I also need to do .as_mut() on it...
    //
    // `as_mut()` returns a `&mut T`, so then I need to deref to get at the `T`
    // value itself, I guess?
    //
    // So I guess `Box` implements `AsMut` and `Box.as_mut()` basically means
    // "hey Box, give me a mutable borrow of your contained value". Then we
    // deref that to modify the value, not the pointer.
    *arg.as_mut() *= *arg.as_mut();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
