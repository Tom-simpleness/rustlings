// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit("ee".to_string()));
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
