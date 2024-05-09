#![allow(dead_code, unused_variables, unused_assignments)]


fn always_return_type() -> i32 {
    5
}

fn block_expr_example() {
    let y = {
        let x = 4;
        x * x
    };

    println!("value of y calculated from expression is: {}", y)
}

fn always_labeled_parameters(a: u32, b: f64) {
    println!("a is {}, and b is {}", a, b);
}

fn proper_func_multiplying_int_and_float(x: i32) -> f32 {
    x as f32 * 1.3
}

pub fn entry_point() {
    let x = always_return_type();

    println!("value of x is {x}");

    block_expr_example();

    always_labeled_parameters(3, 4.0);

    no_forward_declaration_req();

    println!("proper_func_multiplying_int_and_float: {}", proper_func_multiplying_int_and_float(123));

}

fn no_forward_declaration_req() {
    println!("no_forward_declaration_req");
}