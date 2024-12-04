fn main() {
    let mut loop_counter: i8 = 0;

    loop {
        if loop_counter == 10 {
            break;
        }
        loop_counter += 1;
        println!("loop_counter value: {loop_counter}");
    }

    let mut returned_counter = 0;

    let result = loop {
        returned_counter += 1;

        if returned_counter == 10 {
            break returned_counter * 2;
        }
    };

    println!("The result is {result}");

    // Nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Loop into a collection of elements
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
