fn main() {
    // A string slice is a reference to part of a String, and it looks like this:
    // We create slices using a range within brackets by specifying [starting_index..ending_index],
    let s: String = String::from("Hi world");

    let _hello = &s[0..3];
    let _world = &s[4..6];

    let hello_slice = first_word_slice(&s);

    println!("The first word is: {}", hello_slice);

    // s.clear(); // This empties the String, making it equal to ""

    let my_string = String::from("hello world");

    // `first_word_slice` works on slices of `String`s, whether partial or whole
    let _word = first_word_slice(&my_string[0..6]);
    let _word = first_word_slice(&my_string[..]);
    // `first_word_slice` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    // `first_word_slice` works on slices of string literals, whether partial or whole
    let _word = first_word_slice(&my_string_literal[0..6]);
    let _word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_slice(my_string_literal);

    println!("{my_string}");
    println!("Stack address: {:p}", &my_string);
    println!("Heap address: {:p}", my_string.as_ptr());
    println!("Capacity: {}", my_string.capacity());
    println!("Length: {}", my_string.len());
}

// This returned value is a reference to the part of the original String,
// and that’s why it has the type &str. once the original String is dropped,
// the slice is no longer valid.
fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
