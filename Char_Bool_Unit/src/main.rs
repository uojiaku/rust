// Char
// 1
fn size_of_val() {
    use std::mem::size_of_val; // returns the size in bytes of a value
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = 'c';
    println!("{} bytes", size_of_val(&c2));
}
// 2
fn print_char(c: char) {
    println!("{}", c);
}
fn print_above() {
    let c1: char = '字';
    print_char(c1);
}
// Bool
// 3
fn if_statement() {
    let _f: bool = false;
    let t: bool = false;
    if !t {
        println!("Onye ebee ka i bu?");
    }
}
// 4
fn boolean_logic() {
    use std::mem::size_of_val; // bools have 1 byte
    let f: bool = !false;
    let t = true && true;
    assert_eq!(t, f);

    println!("Bools have: {} byte", size_of_val(&f));
}
// Unit type
// 5
fn implicitly_ret_unit() {
    // if a function doesn't return anything, the compiler will implicitly return a unit type
    // unit type is (), an empty tuple. With 0 bytes.
    // use to return "nothing" in expressions or functions
    println!("Daalu rinne");
}

fn explicitly_ret_unit() {
    println!("Daalu rinne!!");
}
fn return_unit_type() {
    let _x: () = ();
    let _v: (i32, i32) = (2, 3);
    assert_eq!(_x, implicitly_ret_unit());
    assert_eq!(_x, explicitly_ret_unit());

    println!("Biko, iwe ewena gi");
}
// 6
fn size_of_unit() {
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); // unit type are of size 0
    println!("kedu udi oru i na-aru?");
}

fn main() {
    size_of_val();
    print_above();
    if_statement();
    boolean_logic();
    return_unit_type();
    size_of_unit();
}
