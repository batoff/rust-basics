fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }
    else {
        b
    }
}

fn main() {
    let max_num = max(10, 412);
    println!("{max_num}");
}
