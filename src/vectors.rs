#![allow(unused_variables, dead_code)]


fn declare_vec() {
    let mut v1: Vec<u32> = Vec::new();

    let v2: Vec<u32> = vec![1, 2, 3, 5];

    let v3 = vec![3, 6, 1, 9]; // Vec<i32>

    v1.push(4);
    v1.push(9);
    v1.push(10);
    v1.push(100);

    for i in 0..v1.len() {
        print!("{}, ", v1[i]);
    }
    println!();


    for i in 0..v1.len() {
        v1[i] = 2 * v1[i];
    }

    println!("{:?}", v1);
}


fn reading_vec() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let third = &mut v[2];

    println!("third elem: {}", third);
    // println!("{:?}", v);

    *third = 10;

    println!("{:?}", v);


    let fourth: Option<&i32> = v.get(3);

}

fn ref_to_vec_elem() {
    let v: Vec<u32> = Vec::new();

    let mut v = vec![1, 2, 3, 4, 6];
    println!("{:?}", v);

    {
        let third: &mut u32 = &mut v[2];
        println!("third: {}", third);
        *third = 100;
    }

    println!("{:?}", v);

    let opt_third: Option<&u32> = v.get(2);
    match opt_third {
        Some(x) => println!("third element is: {}", x),
        None => println!("No third element."),
    }
}

fn out_of_bound() {
    let v: Vec<i32> = vec![3, 6, 1, 9];

    let does_not_exist = v.get(100);
    if does_not_exist == None {
        println!("Index may be out of bounds.");
    }
    // let does_not_exist = &v[100];
    
}

fn iterate_vector() {
    let mut v = vec![3, 5, 6, 8, 9, 12, 24, 65];
    for i in &v {
        print!("{}, ", *i);
    }

    println!();

    for i in &mut v {
        *i *= 10;
    }

    println!("{:?}", v);
}


enum SpreadSheetCell {
    Int(i32),
    Float(f32),
    Text(String)
}

fn enum_vec() {
    let v = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("Gabbar")),
        SpreadSheetCell::Float(2.5),
    ];
}

fn fibo_dp() {

    let n: usize = 47;
    let mut dp: Vec<u32> = Vec::new();

    dp.push(0);
    dp.push(1);
    
    for i in 2..=n {
        dp.push(dp[i-1] + dp[i-2]);
    }

    println!("{:?}", dp);
}

fn vec_of_string_mutate() {
    let mut names = vec!["Japan".to_string(), "Korea".to_string(), "America".to_string()];

    let japan = &mut names[0];

    *japan = "Osaka".to_string();

    println!("{:?}", names);

}


pub fn entry_point() {
    declare_vec();

    reading_vec();

    ref_to_vec_elem();
    out_of_bound();

    iterate_vector();
    enum_vec();

    fibo_dp();

    vec_of_string_mutate();
}