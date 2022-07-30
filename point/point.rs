// have to dervie to print struct that doesn't implement manually Debug for :? syntax
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }
    else {
        b
    }
}

fn print_point(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

fn main() {
    let max_num = max(10, 412);
    println!("{max_num}");

    let p =  Point {
        x: 3,
        y: -6,
    };
    print_point(&p);

    // pretty-print for debug
    println!("{p:#?}");
}

