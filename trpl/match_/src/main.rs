#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Nickel,
    Penny,
    Dime,
    Quarter(UsState),
}
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}{:?}", six, none);
    let dice_roll = 9;
    // 统配模式和_占位符
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _=> reroll()
        // _=> () // 空元组
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

