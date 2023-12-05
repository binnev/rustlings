// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // Whoa you can do tuples of random types??
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // NICE

    let name = parse_cat_name(cat);
    let age = parse_cat_age(cat);

    println!("{} is {} years old.", name, age);
}

fn parse_cat_name(cat: (&str, f32)) -> &str {
    let (name, age) = cat; // no complaining about this unused variable
    return name;
}

fn parse_cat_age(cat: (&str, f32)) -> f32 {
    return cat.1; // tuple elements with dot access?! What about loops?
}
