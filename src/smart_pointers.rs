#![allow(dead_code)]

fn box_intro() {
    let b = Box::new(5);
    println!("b is : {}", b);
}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(_val: i32) -> Self {
        Self {val: _val, next: None }
    }
}


fn normal_dereference() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}

fn smart_pointer_dereference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(*y, 5);
}

struct MyBox<T> (T);


fn nullable_pointer() {
    fn check_optional(input: Option<Box<i32>>) {
        match input {
            None => println!("THere was nothing."),
            Some(bx) => println!("Found: {}", bx),
        }
    }

    let optional = None;
    check_optional(optional);

    let non_optional = Some(Box::new(9000));
    check_optional(non_optional);
}

pub fn entry_point() {
    box_intro();

    normal_dereference();
    smart_pointer_dereference();

    nullable_pointer();
}