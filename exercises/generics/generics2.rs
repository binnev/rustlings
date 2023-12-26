// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> {
    value: T,
}

// This implementation will work for Wrappers of any type V.
// We would usually use the same generic type name as above (T), but we don't have to
impl<V> Wrapper<V> {
    // Whoa, a Self type
    pub fn new(value: V) -> Self {
        Wrapper { value }
    }
}

// _this_ will only be implemented for wrappers of Strings!
impl Wrapper<String> {
    pub fn print(&self) {
        println!("{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
