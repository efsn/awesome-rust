mod matches;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}!", state);
            25
        }
        _ => 0,
    }
}

fn moving() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// -- snip --
fn if_let_demo() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
