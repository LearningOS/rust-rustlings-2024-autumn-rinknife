// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move{x:i32,y:i32},
    ChangeColor(i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move{x:20,y:20});
    println!("{:?}", Message::ChangeColor(0xFF0000));
}
