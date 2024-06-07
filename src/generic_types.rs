#![allow(dead_code, unused_variables)]


fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn print_largest_from_collection() {
    let nums = vec![34, 50, 25, 100, 65];
    let largest = largest_generic(&nums);
    println!("Found largest in vec {:?} is {}", nums, largest);


    let characters = vec!['c', 'b', 'd', 'a', 'x', 'e'];
    let largest = largest_generic(&characters);
    println!("Found largest in vec {:?} is {}", characters, largest);
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic_struct() {
    let integer = Point { x: 4, y: 3 };
    let float = Point { x: 6.7, y: 2.0 };

    // let wont_work = Point { x: 4, y: 5.0 };

    println!("X from integer is: {}", integer.get_x());
}

struct Pointv2<T, U> {
    x: T,
    y: U,
}

fn generic_struct2() {
    let both_integer = Pointv2 { x: 5, y: 10 };
    let both_float = Pointv2 { x: 1.0, y: 4.0 };
    let integer_and_float = Pointv2 { x: 5, y: 4.0 };
}

enum MyOption<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


struct Cell<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Cell<X1, Y1> { // X1 & X2 are generics for the struct
    fn mixup<X2, Y2>(self, other: Cell<X2, Y2>) -> Cell<X1, Y2> { // X2 & Y2 are generics only for the function.
        Cell {
            x: self.x,
            y: other.y,
        }
    }
}


fn generic_mixup() {
    let p1 = Cell { x: 5, y: 3.2 };
    let p2 = Cell { x: 'z', y: "Hello" };

    let p3 = p1.mixup(p2);

    println!("The result of mixup is: x: {}, y: {}", p3.x, p3.y);
}

pub fn entry_point() {
    print_largest_from_collection();

    generic_struct();
    generic_struct2();

    generic_mixup();
}