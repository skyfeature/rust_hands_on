#![allow(dead_code, unused_variables, unused_mut)]

use core::time::Duration;
use std::{collections::HashMap, thread};


#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
enum ShirtColor {
    Red,
    Blue,
}


struct Inventory {
    shirts: Vec<ShirtColor>,
}


impl Inventory {
    fn giveaway(&self, chosen_shirt_color: Option<ShirtColor>) -> ShirtColor {

        // the closure is passed using the or_else, and it is lazily evaluated, meaning the value will calculated only if the Option is None.
        chosen_shirt_color.unwrap_or_else(|| self.most_stocked()) 

        // Can also be done as:
        // match chosen_shirt_color {
        //     Some(chosen_color) => chosen_color,
        //     None => self.most_stocked(),
        // }
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut color_counter = HashMap::new();

        self.count_shirt_colors(&mut color_counter);

        let mut max_count = 0;
        let mut max_color = ShirtColor::Red;
        for (color, count) in &color_counter {
            if count >= &max_count {
                max_color = color.clone();
                max_count = *count;
            }
        }


        println!("{:?}", color_counter);

        max_color
    }

    fn count_shirt_colors(&self, color_counter_map: &mut HashMap<ShirtColor, usize>)  {

        for shirt in &self.shirts {
            let count = color_counter_map.entry(shirt.clone()).or_insert(0);
            *count += 1;
        }
    }
}

fn main_fun() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("User with pref: {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("User with pref: {:?} gets {:?}", user_pref2, giveaway2);

}


fn create_some_closures() {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}


fn closure_and_func() {
    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |y: u32| { y + 1 };
    let add_one_v3 = |z| z + 1;

    let added_one = add_one_v3(5);

    println!("{}", added_one);

    let identity_closure = |x| x;
    let s = identity_closure(String::from("trying closures"));
    // let k = identity_closure(3); // Error since type String has been inferred from above
}


fn immutable_capture() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrow = || println!("from closure: {:?}", list);

    // list.push(5); // cannot, since immutable borrow above and used later.

    println!("Before calling closure: {:?}", list);
    only_borrow();
    println!("After calling closure: {:?}", list);
}

fn mutable_capture() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(5);
    // println!("Before calling closure: {:?}", list); // cannot borrow again immutably in println since already borrowed mutably in the closure.

    borrow_mutably();
    println!("After calling closure: {:?}", list);
}


fn moving_capture() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // closures can outlive the function they have been defined in, like here with thread. list is in the scope of the function. 
    // So, with function moving_capture, list also dies, and thread would be holding invalid reference.
    std::thread::spawn(move || println!("From thread: {:?}", list)) 
        .join()
        .unwrap();

    // println!("After thread execution: {:?}", list); // cannot borrow when the list has been moved to the thread.
}


enum MyOption<T> {
    Some(T),
    None,
}


pub fn entry_point() {
    main_fun();

    create_some_closures();

    closure_and_func();

    immutable_capture();
    mutable_capture();

    moving_capture();
}