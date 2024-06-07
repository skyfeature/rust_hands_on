#![allow(unused_variables, dead_code)]


fn dangling_reference() {
    let dr; // Rust doesn't allow null so it can't be used until initialized.

    // println!("{}", dr); // not allowed here because uninitialized
    {
        let x = "Hello";
        dr = &x; // initialized here
        println!("{}", dr);
    }

    // println!("{}", dr); // dr is dangling reference now. Not allowed.
}

fn no_dangling_reference() {
    let ndr;
    let x = 3;
    ndr = &x;

    println!("{}", ndr);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { // 'a is generic lifetime parameter
    // here we are trying to invoke the relationship between lifetime annotations that, 
    // for some lifetime 'a, both the arguments will stay alive at least till 'a, and the return type will be alive at least 'a

    // by specifying the lifetime, we are not changing the lifetime of any variable, 
    // we are only helping guide the compiler's borrow checker about the lifetime guaranties of variables.

    if s1.len() > s2.len() {
        return s1;
    }
    return s2
}

fn find_longest_string() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let result = longest(&string1, string2);         // works, &String coerced into &str.
    // let result = longest(string1.as_str(), string2); // works, as_str() creates a slice of whole string.
    let result = longest(&string1[..], string2);        // works, manually creating a slice of whole string.

    println!("Bigger one is: {}", result);
}


fn find_longest_string_for_different_lifetime_arguments() {
    let string1 = String::from("I am the long string.");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // works
        println!("The longest string is {}", result);
    }
}

fn find_longest_string_for_different_lifetime_result() {
    let string1 = String::from("longeer string very very");
    let result;

    {
        let string2 = String::from("longer string");

        result = longest(&string1, &string2);

        println!("the longest string is {}", result);
    }

    // println!("the longest string is {}", result); // result stays longer than string2. 
    // But in the function longest we promised that result will stay at least minimum of string1 and string2.
}

fn some_lifetime_examples<'a, 'b>() {
    let x: &i32;
    let y: &'a i32;
    let z: &'b mut i32;
}

fn lifetime_could_be_partially_applied<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn lifetime_does_not_need_to_depend_on_argument<'a>(x: &str, y: &str) -> &'a str { // allowed but
    // let result = String::from("lonnger than everything else");
    // result.as_str() // problem here is result will become a dangling reference
// }

fn lifetime_does_not_need_to_depend_on_argument<'a>(x: &str, y: &str) -> &'a str { // allowed
    let result = "lonnger than everything else";
    result // works
}


struct ImportantExcerpt<'a> { // this means that the instance won't be outliving the reference it is holding in part.
    part: &'a str,
}

fn annotated_struct() {
    let novel = String::from("Call me Ishaemel, from long time agp..");
    let first_sentence = novel.split('.').next().expect("Could not find.");
    {
        let i = ImportantExcerpt {
            part: &first_sentence,
        };
    
        println!("{}", i.part);
    }

    println!("{}", first_sentence);
}


pub fn entry_point() {
    dangling_reference();

    no_dangling_reference();

    find_longest_string();

    some_lifetime_examples();

    find_longest_string_for_different_lifetime_arguments();

    find_longest_string_for_different_lifetime_result();

    annotated_struct();

}

