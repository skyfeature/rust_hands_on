#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        return self.width * self.height;
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}


impl Rectangle {
    fn get_internal() {
        println!("Method giving or taking nothing.");
    }
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool { // &self is alias for self: &Self
        self.area() < rect.area() 
    }
}

impl Rectangle {
    fn is_square(kutta: &Self) -> bool { // this is no longer a struct method
        kutta.width == kutta.height
    }
}

impl Rectangle {
    fn create_square(side: u32) -> Self { // Self is alias for Rectangle inside impl
        Self {
            width: side,
            height: side,
        }
    }
}


fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

pub fn entry_point() {
    let _width = 30;
    let _height = 50;

    let rect = Rectangle {
        width: _width,
        height: _height,
    };

    println!("Area of rectangle is: {}, Perimeter: {}", rect.area(), rect.perimeter());
    Rectangle::get_internal();

    println!("{:?}", rect);

    let rect1 = Rectangle{
        width: 40,
        height: 60,
    };

    let rect2 = Rectangle{
        width: 50,
        height: 80,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sqr = Rectangle::create_square(40);
    println!("{:?} is a square? {}", sqr, Rectangle::is_square(&sqr));


}
