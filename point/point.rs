// have to dervie to print struct that doesn't implement manually Debug for :? syntax
#[derive(Clone, Debug)]
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

fn print_point_from_reference(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}

fn print_point_from_clone(point: Point) {
    println!("x: {}, y: {}", point.x, point.y);
}
    

fn main() {
    let max_num = max(10, 412);
    println!("{max_num}");

    let p =  Point {
        x: 3,
        y: -6,
    };
    print_point_from_reference(&p);
    print_point_from_clone(p.clone());

    // pretty-print for debug
    println!("{p:#?}");
}

