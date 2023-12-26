// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    // This was my original approach. It also kills the self reference.
    // fn append_bar(self) -> Self {
    //     let mut cpy = self.clone(); // move happens here
    //     cpy.push("Bar".to_string());
    //     return cpy;
    // }
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        return self;
    }
}

trait MutateInPlace {
    fn mutate(&mut self);
}

impl MutateInPlace for Vec<String> {
    fn mutate(&mut self) {
        self.push("Bar".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn kill_original() {
        let mut foo = vec![String::from("foo")];

        // for now, I can borrow the original
        assert_eq!(foo, vec![String::from("foo")]);

        // Now I add a value. Because `append_bar` takes ownership of the `self`
        // reference, I need to spit it back out
        foo = foo.append_bar(); // causes move; this _still_ kills the original reference

        // Now this borrow succeeds because we got the reference back
        assert_eq!(foo, vec![String::from("foo"), String::from("Bar")]);
    }

    #[test]
    fn mutate_in_place() {
        let mut original = vec![String::from("foo")];

        // we can borrow the original reference
        assert_eq!(original, vec![String::from("foo")]);

        // we mutate the original
        original.mutate();

        // we borrow again from the same reference... ?
        assert_eq!(original, vec![String::from("foo"), String::from("Bar")]);
    }
}
