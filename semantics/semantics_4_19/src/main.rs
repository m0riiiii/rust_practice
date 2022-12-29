use std::io::Write;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T>(shape: T) where T: HasArea {
    println!("Ths shape has an area of {}", shape.area());
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

fn normal<T: ConvertTo<i64> >(x: &T) -> i64 {
    x.convert()
}

fn inverse<T>() -> T
    where i32: ConvertTo<T> {
        42.convert()
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 1.0};
    println!("{}", c.area());

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(s);

    let mut f = std::fs::File::open("foo.txt").expect("Coudn't open foo.txt");
    let buf = b"whatever";
    let result = f.write(buf);

    println!("{}", inverse());
}
