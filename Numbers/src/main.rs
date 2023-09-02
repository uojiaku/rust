// default number types are:
// integers: i32
// Floats: f64

// signed integer: i8 -> smallest possible 8 bit integer: -255; largest possible 8 bit integer: 255;
// unsigned integer: u8 -> smallest possible 8 bit integer: 0; largest possible 8 bit integer: 255;

// numbers - usize & isize
// architecture dependent:
// on 32-bit architecture -> 32-bit
// on 64-bit architecture -> 64-bit

// one word (pointer sized integer type, matches size of a word in given platform)
// 32-bit processor can access 4 bytes (32 bits) at a time
// 64-bit processor can access 8 bytes (64 bits) at a time

// 1
fn sign_to_unsign() {
    let x: i32 = 5;
    // let mut y : u32 = 5;  -> old
    let mut _y = 5;

    _y = x; // cant assign var from signed to unsigned
    let _z = 10; // default type is i32
    println!("Done!")
}
// 2
fn type_annotations() {
    // other than let x: &str, we can do 'value'u8
    let v: u16 = 38u8 as u16;
    println!("{}", v)
}
// 3
fn type_of<T>(_: &T) -> String {
    // a function to get the type of a given var; return it's string representation
    format!("{}", std::any::type_name::<T>())
}

fn print_type() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("{}", type_of(&x))
}
// 4
fn max_int() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Max Int completed")
}
// 5
fn add_to_max_value() {
    let v1: u16 = 251_u16 + 8; // old -> u8
    let v2: i16 = i16::checked_add(251, 8).unwrap(); // a safer of doing above
    println!("{},{}", v1, v2);
}
//6
fn number_systems_ops() {
    // decimal - hexadecimal - octal - binary
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
                                               // 1_024 - underscore is a delimiter for readability
    assert!(v == 1597);
    println!("arithmetic");
}
// 7
fn print_type_float() {
    let x: f64 = 1_000.000_1; // 1000.0001
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("float");
}
// 8
fn floating_point() {
    // assert!(0.1 + 0.2 == 0.3); // 0.1 + 0.2 = 0.30000000000000002
    // above fails because the floating point is too precise
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // 1st way to fix
    assert!(0.1 as f32 + 0.2 as f32 == 0.3_f32); // 2nd way to fix
    println!("floating success");
}
// 9
fn for_loop() {
    let mut sum: i32 = 0;
    for i in -3..2 {
        // iterates from -3 to 1
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}
// 10
fn range() {
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 }); // assert that 1..5 and Range {start: 1, end: 5} are equal
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // 1..=5 to make 5 inclusive
    println!("{}", "Onye na-ekwu?");
}

// 11
fn computations() {
    // integer addition
    assert!(1u32 + 2 == 3);  // compiler infers 2 and 3 as u32
    // integer subtraction
    assert!(1i32 - 2 == -1); // compiler infers 2 and -1 as i32
    assert!(1i8 - 2 == -1); // old - u8
    assert!(9.6f32 / 3.2 as f32 == 3.0); // 3 different annotations
    assert!(24 % 5 == 4);
    // Boolean Logic
        // Three basic operations - AND, OR, NOT
        // Two Values - true, false
        //  - false AND true = false
        //  - false OR true = true
        //  - !false = true
    // Bitwise operations
        // Operations that manipulate individual bits that make up a binary number
        // Treating each bit of a binary number as a separate unit and perform logical operations on them
        // - AND, OR, XOR, bitwise shifting
            //  XOR -> returns 1 if the inputs are different and 0 if the inputs are the same.
        // Bitwise shifting
        // - in a 8 bit system -> 128 >> 2 = 32 // bitwise rightshift
        // - in a 8 bit system -> 1 << 4 = 16 // bitwise leftshift
    // Short-circuiting boolean logic
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!false == true);

    // Bitwise operations
    println!("0011 AND 0101 = {:04b}", 0b00111u32 & 0b0101);
    println!("0011 OR 0101 = {:04b}", 0b0011u32 | 0b0101);
    println!("00111 XOR 0101 = {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 = {}", 1u32 << 5);
    println!("0x80 >> 2 = 0x{:x}", 0x80u32 >> 2);
}
fn main() {
    sign_to_unsign();
    type_annotations();
    print_type();
    max_int();
    add_to_max_value();
    number_systems_ops();
    print_type_float();
    floating_point();
    for_loop();
    range();
    computations();
}
