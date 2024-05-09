#![allow(unreachable_code, unused_labels)]
#![allow(dead_code, unused_variables, unused_assignments)]

fn branching() {
    let n = 100;
    if n < 0 {
        print!("{n} is negative.\n");
    }
    else if n > 0 {
        print!("{n} is positive.\n");
    }
    else {
        print!("{n} is zero.\n");
    }

    let x = 
        if n < 10 && n > -10 {
            println!("{n} is a small number.");
            n + 10
        }
        else {
            n * n
        };

    println!("{x} is x!");
}


fn checking_loop() {
    let mut count = 0u32;
    println!("count is {count}");

    loop {
        count += 1;

        if count == 3 {
            println!("hurrey, reached 3");
            continue;
        }

        println!("count: {count}");

        if count == 5 {
            println!("Ok, that's enough for today.");
            break;
        }
    }
}


fn check_nested_loops() {
    'outer: loop {
        println!("Entered output loop");

        'inner: loop {
            println!("Entered inner loop.");

            // break;
            break 'outer;
        }

        println!("Outside the inner loop.");

    }

    println!("Outside of outer loop.");
}


fn loop_return() {
    let mut counter = 1;

    let res = loop {

        if counter == 5 {
            break "Hello";
        }
        counter += 1;
    };

    println!("result of the loop is {res}");
}


fn fizz_buzz() {

    let mut n = 1;

    while n < 100 {

        if n % 15 == 0 {
            println!("FizzBuzz");
        }
        else if n % 5 == 0 {
            println!("Fizz");
        }
        else if n % 3 == 0 {
            println!("Buzz");
        }
        else {
            println!("{n}");
        }
        
        n += 1;
    }
}

pub fn entry_point() {
    branching();
    checking_loop();
    check_nested_loops();
    loop_return();
    fizz_buzz();

}
