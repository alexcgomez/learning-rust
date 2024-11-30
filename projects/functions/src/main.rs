fn main() {
    println!("Hello, world!");

    println!("The sum of 1 and 2 is {}", add(1, 2));
    print_labeled_measurement(42, 'm');
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_labeled_measurement(value: i32, unit_label: char) -> () {
    println!("The measurement is: {value}{unit_label}");
}