// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);

    // OK so I can discard / override a variable's type by doing another 'let'
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
