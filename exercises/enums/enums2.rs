// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!


#[derive(Debug)]
// enum 像是一堆struct的集合
enum Message {
    Move { x: i32, y: i32 }, // c风格
    Echo(String),// tuple
    ChangeColor(i32, i32, i32),
    Quit,// unit
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
