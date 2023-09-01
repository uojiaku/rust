// 1
fn first_assert() {
    let x: i32 = 5;
    let _y: i32;
    // will print if assert_eq doesn't break
    assert_eq!(x, 5);
    println!("Nkeoma!");
}
// 2
fn mutable_var() {
    // making variable mutable
    let mut z: i32 = 1;
    z += 2;
    assert_eq!(z, 3);
    print!("Mbido!");
}
// 3
fn scope_vars() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("Kedu ihe nacho na {} maobu oburu kam neme {}", x, y);
    }
    println!("Kedu ihe nacho na {} maobu oburu kam neme {}", x, y);
}
// 4
fn define_x() {
    // type annotation
    let x: &str = "Kedu";
    println!("{}, Uwa!", x);
}
// 5
fn shadowing() {
    // global scoped var
    let x: i32 = 5;
    {
        // local scoped var
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    // redeclaring or shadowing x
    let x = 42;
    println!("{}", x);
}
// 6
fn compile_it() {
    let mut _x: i32 = 1;
    _x = 7;
    // shadowing and re-binding
    // let x = x; // turning mutable x into immutable
    _x += 3;

    let _y = 4;
    // shadowing
    let _y = "I can do anything!";
    println!("Success");
}
// 7
fn allow_unused() {
    let _x = 1; // underscore removes warning
}
// 8
fn destructuring() {
    // you have to make each var mutable in the tuple to increment it
    let (mut x, mut y) = (1, 2); // tuple is destructured
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Complete");
}
// 9
fn destructuring_assignments() {
    // using tuples to destructure assignments
    let (x, y); // same thing as let x; let y;

    (x, ..) = (3, 4); // x = 3, we dont care about 4
    [.., y] = [1, 2]; // y = 2, we dont care about 1

    assert_eq!([x, y], [3, 2]);
    println!("Ngwanu!");
}

fn main() {
    first_assert();
    mutable_var();
    scope_vars();
    define_x();
    shadowing();
    compile_it();
    allow_unused();
    destructuring();
    destructuring_assignments();
}
