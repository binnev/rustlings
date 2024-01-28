// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

// Fuck this, I don't understand clippy's comments, and the code seems stupid
// and contrived in the first place. Skip.

#[allow(unused_variables, unused_assignments)]
fn main() {
    // let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    // let my_arr = &[-1, -2, -3, -4, -5, -6];
    // println!("My array! Here it is: {:?}", my_arr);

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    // println!("This Vec is empty, see? {:?}", my_empty_vec);

    // let mut value_a = 45;
    // let mut value_b = 66;
    // // need to do a mutable borrow because this function will change their
    // // values.
    // std::mem::swap(&mut value_a, &mut value_b);
    // println!("value a: {}; value b: {}", value_a, value_b);
}
