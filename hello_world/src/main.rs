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

/// unit test via cargo
///
/// # examples
/// ```
/// # cargo test
///```
#[test]
fn test_say_hello() {
    print!("test, just say hello");
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("I expected to be panicked");
}

fn main() {
    println!("{}", sum(10, 20));
    println!("Hello, world!");
}
