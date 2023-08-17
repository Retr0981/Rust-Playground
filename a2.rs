fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result)
}

fn main() {
    let result = sum(2, 2);
    display_result(result)
}
