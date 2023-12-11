# Notes 
## Memory safety
### Scopes
- when a variable goes out of scope, rust automatically calls `drop` on it. `drop` calls e.g. String::drop where the author of String can put a routine to free memory. 
- This way you don't have to remember to `free` allocated memory

### References 
- These are like pointers in that they are the address of a piece of memory, and they can be followed to get the value. Unlike pointers they are guaranteed to point to a valid piece of memory for their lifetime (!!!). That's awesome. 
- A reference is denoted by the ampersand. So a reference to a String would be `&String` 
    - bit confusing, given that in C, a pointer to a thing is `*thing`. Or perhaps it's sensible, and C's semantics are confusing. 
    - How many times have I scratched my head over C's pointer being `*thing` but the dereference operator is also `*thing`?
    - remember it like "&and" is an "address" -- they kinda sound the same at the beginning.
    - I guess the operators are the same: 
        - `&thing` gets the address of `thing`
        - `*thing` dereferences to get the value of `thing`
    - references allow functions to *refer* to a value without *owning* it. Because they don't own the value, it doesn't go out of scope when it is passed to the function this way. 
    - references are immutable by default, just like variables; we cannot modify something we have a reference to
        - unless a function receives a `&mut Thing`. Then it can. And it is also very clear from the function signature that it will modify the thing.
        - you can only have one mutable reference to a value at a time (probably for thread safety or something)
        - you also cannot combine mutable and immutable references.
    - A reference's scope starts from where it is introduced and continues through the last time taht reference is *used*. 

# Questions 
- enum return types? Options? Some?
- pattern matching?
- what are macros? How are they different to functions?
- what is borrowing? 
    - borrowing is creating a reference to a value
- What are lifetimes?
- what is moving?
    - when we "reassign" something like a string to a new variable: 
        ```
        let s1 = String::from("hello");
        let s2 = s1;
        ```
    after the assignment to s2, s1 is no longer valid. The memory has been *moved*. (The heap memory allocated to the String, that is)
    - it's kind of like a "shallow copy" in python, in that only the pointer to the heap data is copied, not the heap data itself (except the original variable is invalidated)
    - the `clone()` method can be used to deeply copy data (including the heap data)
    - all this happens for data types like String whose size is not known at compile time. For known-size data types like `i32`, its whole value is stored on the stack and it's very quick to make a copy. So this works without invalidating x: 
    ```
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    ```
- What is copying? 
    - known-size data types can implement the `Copy` trait, which trivially copies their data and allows them to be assigned to other variables. 
    - A data type cannot implement both the `Drop` and `Copy` traits. 
    - when data is passed to a function, it is just like reassignment -- depending on the data's type, the value is either *copied* or *moved*.
        - types that implement `Drop` get moved, and are no longer usable after they've been passed to a function
        - types that implement `Copy` are copied, and can still be used after the function call
- Why the multiple string types? 
    - string literals are immutable
    - String types are for when the value is not known at compile time (like for user input). They *are* mutable