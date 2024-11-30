fn main() {

    /*
    Scalar Types
    - Integers
    - Floating-Point Numbers
    - Booleans
    - Characters
     */

    // Integers
    println!("Integers");
    println!("Signed Integers");

    let integer_8: i8 = 127;
    let integer_16: i16 = 32767;
    let integer_32: i32 = 2147483647;
    let integer_64: i64 = 9223372036854775807;
    let integer_128: i128 = 170141183460469231731687303715884105727;
    let integer_isize: isize = 9223372036854775807;

    println!("i8: {}", integer_8);
    println!("i16: {}", integer_16);
    println!("i32: {}", integer_32);
    println!("i64: {}", integer_64);
    println!("i128: {}", integer_128);
    println!("isize: {}", integer_isize);

    println!("Unsigned Integers");

    let unsigned_integer_8: u8 = 57u8;
    let unsigned_integer_16: u16 = 65535;
    let unsigned_integer_32: u32 = 4294967295;
    let unsigned_integer_64: u64 = 18446744073709551615;
    let unsigned_integer_128: u128 = 340282366920938463463374607431768211455;
    let unsigned_integer_usize: usize = 18446744073709551615;

    println!("u8: {}", unsigned_integer_8);
    println!("u16: {}", unsigned_integer_16);
    println!("u32: {}", unsigned_integer_32);
    println!("u64: {}", unsigned_integer_64);
    println!("u128: {}", unsigned_integer_128);
    println!("usize: {}", unsigned_integer_usize);

    // Floating-Point Numbers
    println!("\nFloating-Point Numbers");

    let float_32: f32 = 3.14;
    let float_64: f64 = std::f64::consts::PI;

    println!("f32: {}", float_32);
    println!("f64: {}", float_64);

    // Numeric Operations
    println!("\nNumeric Operations");
    println!("Addition: 5 + 10 = {}", 5 + 10);
    println!("Subtraction: 95.5 - 4.3 = {}", 95.5 - 4.3);
    println!("Multiplication: 4 * 30 = {}", 4 * 30);
    println!("Division (Quotient): 56 / 7 = {}", 56 / 7);
    println!("Division (Truncated Quotient): -5 / 3 = {}", -5 / 3);
    println!("Division (Remainder): 56 % 7 = {}", 56 % 7);

    // Booleans
    println!("\nBooleans");
    let t = true;
    let f: bool = false;

    println!("t: {}", t);
    println!("f: {}", f);

    // Characters
    println!("\nCharacters");
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    /*
    Compound Types
    - Tuples
    - Arrays
     */

    // Tuples
    println!("\nTuples");
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tuple;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    let empty_tuple: () = ();
    println!("empty_tuple: {:?}", empty_tuple);

    // Arrays
    println!("\nArrays");

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];

    println!("array: {:?}", array);
    println!("first: {}", first);
    println!("second: {}", second);
}
