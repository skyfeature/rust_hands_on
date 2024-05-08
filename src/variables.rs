#![allow(dead_code, unused_variables, unused_assignments)]

use std::io::{self, Read};

const ONE_MINUTE: u32 = 60;


fn reverse(pair_elem: (i32, bool)) -> (bool, i32) {
    let (i_elem, b_elem) = pair_elem;

    return (b_elem, i_elem);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);  // this is tuple struct. Even if composed of same types, two tuple structs are different.

fn check_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (mut x, mut y, mut z) = tup;
    println!("{:?}", tup);

    println!("{}", tup.1);

    x += 1;
    y += 2.9;
    z += 4;

    println!("{:?}", tup);

    let mat = Matrix(3.4, 5.6, 7.5, 0.0);
    let (a, b, c, d) = (mat.0, mat.1, mat.2, mat.3);

    println!("{:?}", mat);

}


fn check_array() {

    let a = [1, 3, 4, 2, 8];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let b: [f64; 3] = [3.2, 5.4, 0.0];

    let carr = [10; 5];
    println!("{:?}", carr);

    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second);
}

fn check_array_bound() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");

    let mut idx = String::new();

    io::stdin().read_line(&mut idx).expect("Failed to read the line.");

    let index: usize = idx.trim().parse().expect("Must be an integer. No spaces.");

    println!("{}", a[index]);
}

pub fn entry_point() {
    
    check_tuple();
    check_array();
    check_array_bound();

    let y: u32;
    let mut x = 5;
    y = x;
    println!("the value of x is {x}, y is {y}");

    x = 6;
    println!("the value of x is {x}, {}", ONE_MINUTE);

    let var = 5;
    let var = var + 1;

    {
        let var = 2 * var;
        println!("var is: {var}");
    }

    println!("var is: {var}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces is: {spaces}!");

    // let mut spaces = "       ";
    // spaces = spaces.len();

    
}
