fn main() {
    // References and Borrowing
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");

        //At any given time, you can have either one mutable reference or any number of immutable references.
        // References must always be valid.
        let dangling = no_dangle();
        println!("No dangle: {dangling}");
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }
}
