#![allow(unused_variables, dead_code)]


fn check_string() {
    
    let str = "Hello"; // string literal. stored in read only data segment

    let mut strn = String::from("World!");

    strn.push_str(" Kesha.");

    println!("{}", strn);
}

#[derive(Debug)]
struct Person {
    age: u32,
    mobile: u32,
}

fn giveup_ownership() {
    let s1 = String::from("Bipul");
    let s2 = s1.clone();

    println!("{}", s1);

    let p1 = Person{ age: 20, mobile: 12345 };
    let p2 = p1;
    // println!("{:?}", p1);

    let x = 5;
    let y = x;
    println!("{}", x);

    let const_str1: &str = "Gold";
    let const_str2 = const_str1;
    println!("{}", const_str1);
}


fn takes_ownership(game: String) {
}

fn makes_copy(players: u32) {
}

fn function_argument_takes_ownership() {
    let cricket = String::from("Cricket");

    let persons = 5;

    takes_ownership(cricket);
    makes_copy(persons);

    // println!("{}-{}", cricket, persons);
}

fn gives_ownership() -> String {
    let s = String::from("Donate me");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn give_and_take_ownership() {

    let s1 = gives_ownership();

    let s2 = String::from("Gentleman");

    let s3 = takes_and_gives_back(s2);

    println!("{}-{}", s1, s3);
}

fn get_length(name: String) -> (String, usize) {

    // (name, name.len())  // borrowing name for length calculation, but it has already moved.
    
    let l = name.len();
    (name, l)
}

fn one_way_to_do_this() {
    let s = String::from("Football");
    let (s, len) = get_length(s);

    println!("{}-{}", s, len);
}

pub fn entry_point() {
    check_string();
    giveup_ownership();
    function_argument_takes_ownership();
    give_and_take_ownership();

    one_way_to_do_this();
}