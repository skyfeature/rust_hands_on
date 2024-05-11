#![allow(dead_code, unused_assignments, unused_variables, unused_mut)]


fn print_string(sref: &String) {
    println!("{}", sref);
}
fn create_ref() {
    let s = String::from("Swimming");
    let s_ref = &s;
    
    print_string(s_ref);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn pass_reference_by_value() {
    let s1 = String::from("Carrom");

    let ln = calculate_length(&s1); // passing reference, but still pass by value.

    println!("{}-{}", s1, ln);
}


fn fix_string(s: &mut String) {
    s.push_str("board");
}

fn modify_using_reference() {
    let mut s1 = String::from("Carrom");

    fix_string(&mut s1);

    println!("{}", s1);
}

fn get_length(s: &String, str: String) {
}

fn test_naive_ref() {
    let s1 = String::from("Yoyo");
    // get_length(&s1, s1);  // cannot move out of `s1` because it is borrowed

}


fn mutable_ref() {
    let s1 = String::from("Khokho");

    let mut s_ref = &s1;

    let s2 = String::from("Kabaddi");
    s_ref = &s2;

    println!("{}, {}-{}", s1, s2, s_ref);
}

fn multiple_refs() {
    let mut s = String::from("Water polo");

    let s1_ref = &s;
    let s2_ref = &s;

    println!("{}, {} & {}", s, s1_ref, s2_ref);
}

fn multiple_mutable_refs_not_allowed() {
    let mut s = String::from("Baseball");
    let ref1 = &mut s;
    // let ref2 = &mut s;

    // println!("{}: {} & {}", s, ref1, ref2);
}


fn mixing_mutable_immutable_refs_not_allowed() {
    let mut s = String::from("Baseball");
    let ref1 = &s;
    let ref2 = &s;
    // let ref3 = &mut s;

    // println!("{}: {} & {} & {}", s, ref1, ref2, ref3);
}

fn mixing_mutable_immutable_refs_allowed_when_one_of_the_types_goes_out_of_scope() {
    let mut s = String::from("Baseball");
    let ref1 = &s;
    let ref2 = &s;

    println!("{}: {} & {}", s, ref1, ref2); // compiler deduces that ref1 and ref2 goes out of scope here.

    let ref3 = &mut s;

    println!("Works: {}", ref3);
    // println!("Fails: {}-{}", s, ref3);
}

fn bar1(x: &mut i32) {}
fn foo1(a: &mut i32) {
    let y = &a;
    // bar1(a);
    println!("{}", y);
}

fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    bar(a);
    let y = &a;
    println!("{}", y);
}


fn goes_out_of_scope_ref() {
    let mut s = String::from("Laps");
    {
        let ref1 = &mut s;
    }
    let ref2 = &s;
}

// fn dangle() -> &String {
//     let s = String::from("Vollyball");
//     &s
// }
fn dangling_refrences() {

    // let dang_ref = dangle();
}


pub fn entry_point() {
    create_ref();
    pass_reference_by_value();
    modify_using_reference();
    test_naive_ref();
    
    mutable_ref();

    multiple_refs();

    multiple_mutable_refs_not_allowed();
    mixing_mutable_immutable_refs_not_allowed();
    mixing_mutable_immutable_refs_allowed_when_one_of_the_types_goes_out_of_scope();
    goes_out_of_scope_ref();

    dangling_refrences();
}