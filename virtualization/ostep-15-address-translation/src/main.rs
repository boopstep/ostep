fn main() {
    let mut x: i16 = 10;
    let mut y: i32 = 10;

    let x_addr: *const i16 = &x;
    let y_addr: *const i32 = &y;

    println!("{:?}", x_addr);
    println!("{:?}", y_addr);

    x = x + 3;
}
