enum Expr {
    Null,
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div { dividend: i32, divisor: i32 },
    Val(i32),
}

fn print_expr(expr: Expr) {
    match expr {
        Expr::Null => println!("No value"),
        Expr::Add(x, y) => println!("{}", x + y),
        Expr::Sub(x, y) => println!("{}", x - y),
        Expr::Mul(x, y) => println!("{}", x * y),
        Expr::Div{ dividend: x, divisor: 0 } => println!("Divisor is zero"),
        Expr::Div{ dividend: x, divisor: y } => println!("{}", x / y),
        Expr::Val(x) => println!("{}", x)
    }
}

fn uppercase(c: u8) -> u8 {
    if let b'a'..=b'z' = c {
        c - 32
    }
    else {
        c
    }
}

fn is_alphanumeric(c: char) -> bool {
    match c {
        // inclusive range
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false,
    }
}

fn main() {
    let quotient = Expr::Div{ dividend: 10, divisor: 2 };
    let sum = Expr::Add(40, 2);
    let null = Expr::Null;
    let div_err = Expr::Div{ dividend: 12, divisor: 0 };
    
    print_expr(quotient);
    print_expr(sum);
    print_expr(null);
    print_expr(div_err);
}
