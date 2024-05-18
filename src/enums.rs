#![allow(unused_variables, dead_code, unused_assignments)]


#[derive(Debug)]
enum MyIpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl MyIpAddr {
    fn call(&self) {
        
        match self {
            Self::V4(a, b, c, d) => println!("{}, {}, {}, {}", a, b, c, d),
            Self::V6(s) => println!("{}", s),
        }
    }
}

fn testing_enum_obj() {
    let home = MyIpAddr::V4(127, 0, 0, 1);
    let loopback = MyIpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    home.call();
    loopback.call();
}

fn checking_option() {
    let x_opt = Some(5);
    let y_opt = Some(String::from("Hello"));

    let no_val_opt: Option<f32> = None;
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn testing_enum_method() {
    let coin = Coin::Dime;
    coin.value_in_cents();
}

// Option is an enum
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Behaviour {
    Exempt,
    NotExempt,
}

enum Side {
    Buy,
    Sell,
    ShortSell(Behaviour),
}

fn get_side(side: &Side) -> &str {
    match side {
        Side::Buy => "buy",
        Side::Sell => "sell",
        Side::ShortSell(behaviour) => {
            println!("Short sell type is: {:?}", behaviour);
            "short_sell"
        },
    }
}

fn test_pattern_matching() {
    let sd = Side::ShortSell(Behaviour::NotExempt);
    let result = get_side(&sd);
    println!("Found side: {}", result);

    // println!("Exempt has default value 0: {}", Behaviour::Exempt == 0); // cannot compare like this
}


fn plus_one(value: Option<u32>) -> Option<u32> {
    match value {
        None => None,
        Some(x) => {
            println!("Value in Option<i32>: {}", x);
            Some(x + 1)
        },
    }
}

fn match_with_option() {
    let five: Option<u32> = Some(5);
    let six = plus_one(five);
    let no_one = plus_one(None);
}


fn catch_all_pattern() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let button = "red";
    match button {
        "green" => add_fancy_hat(),
        "blue" => remove_fancy_hat(),
        _ => reroll(),
    }

    let country = "India";
    match country {
        "Thailand" => println!("Land of smiles!"),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(x: u8) { println!("move player position by {}", x) }
fn reroll() {}


fn if_let_control_flow() {

    let config_max = Some(3u8);
    match config_max {
        Some(max_val) => println!("Max value for config using match is {}", max_val),
        _ => (),
    }

    if let Some(max_val) = config_max { // useful if we want to match only one pattern and ignore rest.
        println!("Max value for config using if let is: {}", max_val);
    }

    let coin = Side::ShortSell(Behaviour::Exempt);
    let mut other = 1;
    if let Side::ShortSell(behav) = coin {
        println!("Behaviour for short sell is: {:?}", behav);
    }
    else {
        other += 1;
    }

}

pub fn entry_point() {
    testing_enum_obj();

    checking_option();

    testing_enum_method();

    test_pattern_matching();

    match_with_option();

    catch_all_pattern();

    if_let_control_flow();
}
