enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // tuple of i32
}

impl Message {
    fn call(&self) {}
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    value_in_cents(Coin::Nickel);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
