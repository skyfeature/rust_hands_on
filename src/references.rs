#![allow(dead_code)]

use std::{mem::take, ptr};

fn take_ref(num: &String) {
    println!("{}", num);
}


fn testing_pass_by_copy() {
    let x = String::from("Hello x");

    let x_ref = &x;
    println!("x_ref is: {}", x_ref);

    take_ref(x_ref);

    println!("original x_ref: {}", x_ref);
}

fn reference_comparisions() {
    let five = 5;
    let five_ref = &five;
    let same_five_ref = &five;
    
    let other_five = 5;
    let other_five_ref = &other_five;

    assert!(five_ref == same_five_ref);
    assert!(five_ref == other_five_ref);

    assert!(ptr::eq(five_ref, same_five_ref));
    assert!((!ptr::eq(five_ref, other_five_ref)));
}

pub fn entry_point() {
    reference_comparisions();

    testing_pass_by_copy();
}