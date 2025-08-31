static X: i32 = 1;

static SQUARE: fn(i32) -> i32 = |x| x * x;
static ADD: fn(i32, i32) -> i32 = |x, y| x + y;

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let x = 1;
    let mut y = 1;
    println!("Hello, world!");
    println!("The value of X is: {}", X);
    println!("The value of y is: {}", y);
    y = 2;
    println!("The value of y is: {}", y);
    y = SQUARE(y);
    println!("The value of y is: {}", y);
    y = ADD(y, 2);
    println!("The value of y + 2 is: {}", y);
    println!("The value of 3 * 2 is: {}", multiply(3, 2));
}
