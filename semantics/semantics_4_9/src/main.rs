struct Foo<'a> {
    x: &'a i32,
}

static FOO: i32 = 5;

fn main() {
    let x = 1;

    {
        let y = &5;
        let f = Foo {x: y};
        // this is error! borrowed value does not live long enough
        //x = &f.x;
    }

    println!("{}",x);


    let x: &'static i32 = &FOO;
    println!("{}",x);
}

// 黙示的に
fn _foo(_x: &i32) {
}

// 明示的に
fn _bar<'a>(_x: &'a i32) {
}
