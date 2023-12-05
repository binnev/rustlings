// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    // looks like we're doing a sort of multi-level variable assignment here. I
    // wonder if it's a specific construct, or if the whole "if" block can be
    // evaluated to a value in other contexts, too.
    // It's a bit like python's `foo = 1 if ... else 2` but more readable,
    // because the order isn't reversed
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        420
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
