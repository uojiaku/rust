// Diverging Functions
// - Never return to the caller
// - E.g. panic, looping forever, quitting the program
// 1
fn sum(x: i32, y: i32) -> i32 {
    // you have to give x and y and return a type
    x + y
}
fn v1() {
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("{}", s);
}
// 2
fn print() -> () {
    // replaced i32 with ()
    println!("Ya gaziere gi taa");
}
fn v2() {
    print();
}
// 3
fn never_return() -> ! {
    // implement this function, don't modify the fn signatures
    panic!() // panic macro causes the program to panic (programs exits and returns an error)
}
fn v3() {
    never_return();
    println!("Failed!"); // this never runs
}
// Diverging functions may be used in places where a value of any type is expected.
// 4
fn never_return_fn() -> ! {
    panic!()
    // todo!() // or
    // unimplemented!() // or
}
// never called
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // todo
        }
        _ => {
            // todo
        }
    };
    // Rather than returning a None, we use a diverging function instead
    never_return_fn();
}
fn v4() {
    println!("I ghotara");
}
// 5
fn v5() {
    let b = false;
    let _v = match b {
        true => 1,
        // diverging functions can also be used in match expression to replace a value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic"); // we can give a panic msg
        }
    };
    println!("Exercise Failed if printing out this line!");
}

fn main() {
    v1();
    v2();
    // v3(); // commented out because it causes panic
    v4();
    v5();
}
