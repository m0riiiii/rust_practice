struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut origin = Point {x: 0, y:0 };

    origin.x = 5;

    println!("The origin is at ({},{})",origin.x, origin.y);

    let mut point = Point3d { x:0, y:0, z:0 };
    point = Point3d { y:1, .. point};

    println!("point x:{}, y:{}, z:{}",point.x,point.y,point.z);
}
