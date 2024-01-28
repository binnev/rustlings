// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// Macros are not functions -- they are code expansion / substitution that is
// done at _compile time_. So the below macro is equivalent to the compiler
// copy-pasting `println!("Check out my macro!")` into every place where we call
// `my_macro~()` in the source code.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
