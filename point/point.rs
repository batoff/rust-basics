// have to dervie to print struct that doesn't implement manually Debug for :? syntax
#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self { // instead of Self can also be Point
        Self { x, y }
    }

    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt() // cast sum from int to f64, because sqrt() is defined on floats
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
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

    let mut p =  Point::new(3, -6);
    print_point_from_reference(&p);
    inc_x(&mut p);
    print_point_from_clone(p.clone());
    
    p.translate(-2, 10);
    // pretty-print for debug
    println!("{p:#?}");

    println!("dist from origin - {}", p.dist_from_origin());
    
    let tuple = (23, 61);
    println!("{}, {}", tuple.0, tuple.1);

    let (hello, world) = "helloworld".split_at(5);
    println!("{}, {}!", hello, world);
}

