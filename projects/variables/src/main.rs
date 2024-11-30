fn main() {
    const SPEED_OF_LIGHT: f64 = 299_792_458.0;
    println!("The speed of light is: {SPEED_OF_LIGHT} m/s");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let shadowed_variable = 0;
    println!("The first value of shadowed_variable is: {shadowed_variable}");

    // Inner scope
    {
        let shadowed_variable = shadowed_variable + 10;
        println!("The value of shadowed_variable is: {shadowed_variable}");
    }

    let shadowed_variable = shadowed_variable + 1;
    println!("The second value of shadowed_variable is: {shadowed_variable}");
}