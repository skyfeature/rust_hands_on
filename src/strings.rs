#![allow(unused_mut, unused_variables)]


fn string_creation() {
    let mut s = String::new();

    let d: &str = "North";
    let dir: String = d.to_string();

    let food: String = "Cupcake".to_string();

    let mut sport = String::from("Soccer");

    sport.push('|');
    sport.push_str(d);
    sport.push('|');
    sport.push_str(&food);

    println!("sport is: {}", sport);
}


fn string_concat() {
    let food = String::from("cheese");
    let dir = String::from("East");

    let mixture = food + &dir;  // fn add(self, &str) -> String. food is moved to add.
    // since add takes &str and we are passing &String, compiler will coarse &dir into &dir[..]
    // println!("{} and {}", mixture, food);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let name = s1 + "-" + &s2 + "-" + &s3; // s1 got moved
    println!("{}", name);

    let s1 = "tic".to_string();
    let tictactoe = format!("{s1}-{s2}-{s3}"); // everything borrowed, nothing moved.
    println!("{}", tictactoe);

    println!("{s1} & {s2} & {s3}");
}

fn string_index() {
    // no such thing in Rust.
}

fn string_slicing() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // works only if we slice off at char boundaries otherwise Rust panics.
}

fn iterate_string() {
    let hello = "नमस्ते"; // "Здравствуйте";

    for c in hello.chars() {
        print!("{c} ");
    }
    println!();

    for c in hello.bytes() {
        print!("{c} ");
    }
    println!();
}

pub fn entry_point() {
    string_creation();

    string_concat();

    string_index();

    string_slicing();

    iterate_string();
}