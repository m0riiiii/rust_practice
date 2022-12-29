struct Point<T,U> {
    x: T,
    y: U,
}

impl<T> Point<T,T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x,&mut self.y);
    }
}

fn main() {
    let x: Option<i32> = Some(5);
}
