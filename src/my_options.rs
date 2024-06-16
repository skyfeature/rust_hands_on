#![allow(dead_code)]

fn add_two_numbers(list: &mut Vec<i32>) -> Option<i32> {
    let p = list.pop();
    let q = list.pop();

    match (p, q) {
        (Some(r), Some(s)) => Some(r + s),
        _ => None,
    }
}

fn check_if_value() {
    let a = Some(900);
    assert_eq!(a.is_none(), false);
    assert_eq!(a.is_some(), true);
}

fn add_two_numbers_q(list: &mut Vec<i32>) -> Option<i32> {
    return Some(list.pop()? + list.pop()?);
}

fn convert_result_into_option() {
    let res: Result<i32, String> = Result::Ok(5);
    let opt_res = res.ok();
    assert_eq!(opt_res, Some(5));

    let res: Result<i32, &str> = Result::Err("Missing something");
    let opt_res = res.ok();
    assert_eq!(opt_res, None);
}

pub fn entry_point() {
    check_if_value();
    convert_result_into_option();
}