// Statement:
// - instructions that perform some action but do not produce a value
// - function definitions are statements, as well as code that ends with ";" (usually)
// Expression:
// - evaluate to a resultant value
fn initialize_var() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // this expression will be assigned to `y` because it has no ';'
        x_cube + x_squared + x
    };
    let z = {
        // the semicolon suppresses this expressoin and '()' is assigned to 'z'
        2 * x;
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
// 1
fn v1() {
    let v = {
        let mut x = 1;
        x += 2; // -> variable assignments are statements, so if w/o ';' it returns ()
        x // returns x
    };

    assert_eq!(v, 3);
    println!("Ya gaziere gi taa!");
}
// 2
fn v2() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);
    println!("{}", v);
}
// 3
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
fn v3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);
    println!("the sum result is -> {}", s)
}

fn main() {
    initialize_var();
    v1();
    v2();
    v3();
}
