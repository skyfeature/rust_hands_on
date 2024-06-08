#![allow(unused_imports)]

use crate::exercises::get_median_mode;
use crate::exercises::great_string;

mod functions;
mod variables;
mod branching;
mod guessing_game;
mod structs;
mod ownership;
mod borrowing;
mod slicing;
mod vectors;
mod crashes;
mod rectangles;
mod enums;
mod strings;
mod hashmaps;
mod exercises;
mod closures;
mod oops;
mod generic_types;
mod references;
mod traits;
mod lifetimes;
mod my_iterators;
mod matches;

fn main() {
    println!("Hello, world!");

    // variables::entry_point();
    // functions::entry_point();
    // branching::entry_point();
    // guessing_game::entry_point();
    // structs::entry_point();
    // ownership::entry_point();
    // borrowing::entry_point();
    // slicing::entry_point();
    // vectors::entry_point();
    // crashes::entry_point();
    // rectangles::entry_point();
    // enums::entry_point();
    // strings::entry_point();
    // hashmaps::entry_point();

    // get_median_mode::entry_point();
    // closures::entry_point();
    // oops::entry_point();
    // generic_types::entry_point();
    // references::entry_point();

    // traits::entry_point();

    // lifetimes::entry_point();

    // great_string::entry_point();

    my_iterators::entry_point();

    matches::entry_point();
}
