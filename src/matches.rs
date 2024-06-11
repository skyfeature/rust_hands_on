#![allow(dead_code)]

fn foo() {
    println!("foo");
}

fn bar() {
    println!("bar");
}

// match arms must be same type
// fn bar() -> String {
//     "Hello".to_string() 
// }

pub fn entry_point() {
    let status_foo = [1, 2, 4, 11, 27, 88]; // the type is array[i32, 6]
    let status_bar = [3, 5, 6, 7, 8, 9];
    let status = 2;
    match status {
        status if status_foo.contains(&status) => foo(),
        status if status_bar.contains(&status) => bar(),
        _ => (),
    };
}