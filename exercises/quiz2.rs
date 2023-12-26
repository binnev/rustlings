// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // I want the mutable string type, because
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        // This is cool; I can use `match` in the middle of a function call
        for (string, command) in input.iter() {
            output.push(match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                // Another alternative:
                // Command::Append(n) => string.to_owned() + &"bar".repeat(*n)
                // `string` is type &String, so I need to convert it to owned so I can modify it.
                // Then I immediately need to borrow my repeated string literal for some reason.
                // This approach looks confusing so I went for `format` instead
                Command::Append(n) => {
                    let bars = "bar".repeat(*n); // this is type &str I think
                    format!("{string}{bars}") // format creates a new string, leaving both inputs untouched.
                }
            })
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            // I think the `.into()` here just converts this string literal
            // into whatever string type I specified in my function signature.
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
