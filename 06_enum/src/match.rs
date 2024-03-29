use std::{option, i32};

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny !");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(1+i),
        None => None,
    }
}


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    println!("{:#?}", none);

    let dice_roll: u8 = 3;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    }

    println!("{:?}", opt)

    
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}