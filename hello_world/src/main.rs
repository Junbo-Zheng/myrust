/// used for add two nums
///
/// # examples
/// ```
/// let x = sum(1,2);
/// ```
fn sum(a: i32, b: i32) -> i32 {
    println!("sum for a + b = {}", a + b);
    // let s = a + b;
    // s
    return a + b;
}

fn main() {
    println!("{}", sum(10, 20));
    println!("Hello, world!");
}
