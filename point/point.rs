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
 
fn inc_x(point: &mut Point) {
    point.x += 1;
}

fn main() {
    let max_num = max(10, 412);
    println!("{max_num}");

    let mut p =  Point {
        x: 3,
        y: -6,
    };
    print_point_from_reference(&p);
    inc_x(&mut p);
    print_point_from_clone(p.clone());

    // pretty-print for debug
    println!("{p:#?}");
}

