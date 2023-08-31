fn define_x() {
    // type annotation
    let x: &str = "Kedu";
    println!("{}, Uwa!", x);
}

fn main() {
    let x: i32 = 5;
    let _y: i32;
    // will print if assert_eq doesn't break
    assert_eq!(x, 5);
    println!("Nkeoma!");

    // making variable mutable
    let mut z: i32 = 1;
    z += 2;
    assert_eq!(z, 3);
    print!("Mbido!");

    define_x();
}
