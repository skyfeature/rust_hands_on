#![allow(dead_code, unused_variables, unused_must_use)]

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn panicked() {
    panic!("crash and burn");
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn handle_file() {
    let missing_file_result = File::open("no such file.txt");

    let result = match missing_file_result {
        Ok(fh) => fh,
        Err(error) => panic!("File is missing {:?}", error),
    };

}

fn handle_file_error() {
    let file_name = String::from("hello.txt");
    let greeting_file_result = File::open(&file_name);
    let greeting_file = match greeting_file_result {
        Ok(fh) => fh,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(nfh) => nfh,
                Err(e) => panic!("Problem creating the file: {}, received error: {}", &file_name, e),
            },
            other_error => panic!("Problem opening the file: {}, received error: {}", file_name, other_error),
        }
    };
}

fn handle_file_error_using_closure() {
    let file_name = String::from("hello.txt");
    let file_handle = File::open(&file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_name).unwrap_or_else(|fc_err| {
                panic!("Problem creating the file: {}, received error: {:?}", file_name, fc_err);
            })
        }
        else {
            panic!("Problem opening the file: {}, received error: {}", file_name, error);
        }
    });
}

fn handle_file_unwrap_calls_panic() {
    let file_handle = File::open("no_file.txt").unwrap();
}

fn handle_file_expect_choose_own_panic_msg() {
    let fh = File::open("no_file.txt").expect("The file should be present");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let fh_result = File::open("username.file");
    let mut fh = match fh_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username_data = String::new();
    let read_result = fh.read_to_string(&mut username_data);
    match read_result {
        Ok(_) => Ok(username_data),
        Err(read_error) => Err(read_error),
    }
}

fn error_propagation() {
    let res = read_username_from_file().expect("We should get an username from the file");
}


fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("Myfile.txt")?;
    let mut username_data = String::new();

    username_file.read_to_string(&mut username_data)?;
    Ok(username_data)
}


fn error_propagation_shortcut() {
    let res = read_username_from_file_shortcut().expect("We should have received username in the file.");
}


fn read_username_from_file_chained_propagation() -> Result<String, io::Error> {
    let mut username_data = String::new();

    File::open("Myfile.txt")?.read_to_string(&mut username_data)?;
    Ok(username_data)
}

fn chained_error_propagation() {
    let res = read_username_from_file_chained_propagation().expect("File should have the username");
}

fn read_file_using_inbuild_fn() -> Result<String, io::Error> {
    fs::read_to_string("myfile.txt")
}


fn interrogation_mark() {
    // let fh = File::open("myfile.txt")?; // here the result Err is returning the error but fn is not set to return anything.
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn interrogation_mark_with_option() {
    last_char_of_first_line("foo\r\nbar\n\nbaz\r");

    last_char_of_first_line("\nhi");
}


pub fn entry_point() {
    // panicked();
    // handle_file();
    handle_file_error();
    handle_file_error_using_closure();

    // handle_file_unwrap_calls_panic();
    // handle_file_expect_choose_own_panic_msg();

    // error_propagation();
    // error_propagation_shortcut();
    // chained_error_propagation();

    read_file_using_inbuild_fn();
    interrogation_mark();

    interrogation_mark_with_option();
}