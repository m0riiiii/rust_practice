struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }

    fn x(&mut self, cordinate: f64) -> &mut CircleBuilder {
        self.x = cordinate;
        self
    }

    fn y(&mut self, cordinate: f64) -> &mut CircleBuilder {
        self.y = cordinate;
        self
    }

    fn radius(&mut self, cordinate: f64) -> &mut CircleBuilder {
        self.radius = cordinate;
        self
    }

    fn finalize(&self) -> Circle {
        Circle {x: self.x, y: self.y, radius: self.radius}
    }

}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}",c.area());

    println!("{}",c.grow(2.0).area());

    println!("{}", Circle::new(1.0, 1.0, 2.0).area());

    let d = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();

    println!("{}",d.area());
}
