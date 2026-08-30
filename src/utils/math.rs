use crate::standard_utils;

pub fn sm() {

    let mut n1 = String::new();
    standard_utils::get_stdin_method("Enter first number: ", &mut n1);
    let int_n1 = standard_utils::str_to_i32(&mut n1);

    let mut n2 = String::new();
    standard_utils::get_stdin_method("Enter second number: ", &mut n2);
    let int_n2 = standard_utils::str_to_i32(&mut n2);
    let result;
    result = int_n1 + int_n2;
    println!("the result of your operation was: {}", result);
}

pub fn sb() {

    let mut n1 = String::new();
    standard_utils::get_stdin_method("Enter first number: ", &mut n1);
    let int_n1 = standard_utils::str_to_i32(&mut n1);

    let mut n2 = String::new();
    standard_utils::get_stdin_method("Enter second number: ", &mut n2);
    let int_n2 = standard_utils::str_to_i32(&mut n2);
    let result;
    result = int_n1 - int_n2;
    println!("the result of your operation was: {}", result);
}

pub fn mul() {

    let mut n1 = String::new();
        standard_utils::get_stdin_method("Enter first number: ", &mut n1);
    let int_n1 = standard_utils::str_to_i32(&mut n1);

    let mut n2 = String::new();
        standard_utils::get_stdin_method("Enter second number: ", &mut n2);
    let int_n2 = standard_utils::str_to_i32(&mut n2);
    let result;
    result = int_n1 * int_n2;
    println!("the result of your operation was: {}", result);
}
