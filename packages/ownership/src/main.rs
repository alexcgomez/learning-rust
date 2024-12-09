fn main() {
    // s1 is moved before use it, and its memory is freed
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");

    // Initial Heap memory unused
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // s1 is deep copied into s2 so there are two heap memory spaces used
    let s1 = String::from("hello2");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    /*
    x and y are shallow copied, not deeply copied, and we don’t have a call to clone explicitly.
    The reason is that types such as integers that have a known size at compile time are
    stored entirely on the stack, so copies of the actual values are quick to make.

    Simple scalars implement the Copy trait, meaning that since their memory layout is fixed
    and known at compile time, they can be copied efficiently in the stack without requiring
    heap allocation.
    */
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    // Ownership and Functions

    // Ownership is also passed to functions
    {
        {
            let s = String::from("hello"); // s comes into scope

            takes_ownership(s); // s's value moves into the function...
                                // ... and so is no longer valid here

            // println!("{s}"); // Error: value borrowed here after move

            let x = 5; // x comes into scope

            makes_copy(x); // x would move into the function,
                           // but i32 is Copy, so it's okay to still
                           // use x afterward
        } // Here, x goes out of scope, then s. But because s's value was moved, nothing
          // special happens.

        fn takes_ownership(some_string: String) {
            // some_string comes into scope
            println!("{some_string}");
        } // Here, some_string goes out of scope and `drop` is called. The backing
          // memory is freed.

        fn makes_copy(some_integer: i32) {
            // some_integer comes into scope
            println!("{some_integer}");
        } // Here, some_integer goes out of scope. Nothing special happens.
    }

    // Return Values and Scope
    {
        {
            let s1 = gives_ownership(); // gives_ownership moves its return
                                        // value into s1

            let s2 = String::from("hello"); // s2 comes into scope

            let s3 = takes_and_gives_back(s2); // s2 is moved into
                                               // takes_and_gives_back, which also
                                               // moves its return value into s3
        } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
          // happens. s1 goes out of scope and is dropped.

        fn gives_ownership() -> String {
            // gives_ownership will move its
            // return value into the function
            // that calls it

            let some_string = String::from("yours"); // some_string comes into scope

            some_string // some_string is returned and
                        // moves out to the calling
                        // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String {
            // a_string comes into
            // scope

            a_string // a_string is returned and moves out to the calling function
        }
    }

    /*    References and Borrowing
    Rust does let us return multiple values using a tuple, as shown in Listing 4-5.
    But this is too much ceremony and a lot of work for a concept that should be common.
    Luckily for us, Rust has a feature for using a value without transferring ownership,
    called references.
    */
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String

            (s, length)
        }
    }
}
