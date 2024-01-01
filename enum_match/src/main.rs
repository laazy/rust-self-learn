use rand::Rng;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

#[derive(Debug)]
enum State{
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn get_random_coin() -> Coin {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => Coin::Penny,
        1 => Coin::Nickel,
        2 => Coin::Dime,
        3 => Coin::Quarter(State::Alabama),
        _ => Coin::Quarter(State::Alaska),
    }
}
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6;
    println!("four [0]: {}", match four {
        IpAddrKind::V4(a, _, _, _) => a,
        _ => 0,
    });
    dbg!(four);
    dbg!(six);

    let coin = get_random_coin();
    println!("coin:{:#?}, value: {}",&coin, value_in_cents(&coin));
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => println!("Not a state quarter!"),
    }
    // println!("coin:{:#?}",coin);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a state quarter!");
    }

    let s = Option::Some(5);
    match s {
        Option::Some(i) => println!("i: {}", i),
        _ => (),
    }
    if let Option::Some(i) = s {
        println!("i: {}", i);
    }
    // println!("coin:{:#?}",coin);
    // println!("value: {}", value_in_cents(&coin));

}
