// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

// So this is just the _description_ of the desired interface. The blueprint,
// without a concrete implementation.
trait AppendBar {
    fn append_bar(self) -> Self;
}

// Here we
// 1) provide the concrete implementation of AppendBar and
// 2) attach it to String.
//
// And, I guess, mark `String` as having implemented AppendBar
//
// Let's just pause for a second. We _attached new functionality to a std
// library object!_ That's awesome.
impl AppendBar for String {
    fn append_bar(self) -> Self {
        // We return a new copy (?)
        self.to_owned() + "Bar"
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
