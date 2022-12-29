enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move{ x: i32, y: i32 },
    Write(String),
}

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

fn main() {
    let x: Message = Message::Move {x: 3, y: 4};

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    let m = Message::Write("Hello world".to_string());

    let v = vec!["hello".to_string(), "world".to_string()];
    let vl: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
