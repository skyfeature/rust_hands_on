#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Empty;

struct TwoElem(i32, f64);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn checking_tuple() {

    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let mut ravi = Person {
        name: String::from("Ravi Dev"),
        age: 2,
    };

    ravi.name = String::from("Ravi Yadav");

    let p1 = Point {
        x: 10.1, 
        y: 23.1,
    };
}

fn create_person(username: String, _age: u8) -> Person {
    Person{name: username, age: _age}
}

fn create_person2(username: String, _age: u8) -> Person {
    return Person{name: username, age: _age}
}

fn create_person3(username: String, _age: u8) -> Person {
    return Person{name: username, age: _age};
}

fn create_person4(username: String, _age: u32) -> Person {
    return Person{name: username, age: _age as u8};
}

fn check_mismatch_type_op() {
    let a: u32 = 100;
    let b: i32 = 200;

    println!("a + b is {}", a as i64 + b as i64);
}

fn tuple_structs() {
    struct Space(u32, u32, u32);
    struct Color(u32, u32, u32);

    let chess_board = Space(10, 12, 14);
    let red = Color(255, 0, 0);

    let (a, b, c) = (red.0, red.1, red.2);
    println!("{}, {}, {}", a, b, c);

}

fn unit_tuple_struct() {
    struct AlwaysEqual;

    let equaliti = AlwaysEqual;

}

#[derive(Debug)]
struct Identity {
    email: String,
    age: u32,
}

fn create_user(email_id: String, age_num: u32) -> Identity {
    let user = Identity{
        email: email_id,
        age: age_num,
    };

    // println!("{}, {}", email_id, age_num);  // fails since email_id has already been moved.

    user
}

fn test_move() {
    let email = String::from("abc@gmail.com");

    let user1 = create_user(email, 20);
    println!("{:?}", user1);
}




pub fn entry_point() {

    checking_tuple();
    check_mismatch_type_op();
    tuple_structs();

    unit_tuple_struct();

    let text = String::from("Ram");
    let mut ram = create_person(text, 20);

    ram.name = String::from("Tejas");

    println!("{:?}", ram);


    test_move();
    
}