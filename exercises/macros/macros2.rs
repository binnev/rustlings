// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// The macro has to go before the main function. I guess because this is at
// compile time so the compiler is going over everything in order. It's a bit
// like C's function prototypes though and I don't really like that...

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
