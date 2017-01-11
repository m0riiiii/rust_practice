struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!")
    }
}

struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {}!!!",self.strength);
    }
}

fn main() {
    let x = HasDrop;
    // dropメソッドはtntから呼ばれる
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
}
