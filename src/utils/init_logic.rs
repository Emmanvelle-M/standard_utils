use crate::standard_utils;

use crate::utils::loop_q;

use crate::utils::math;

pub fn quizz() {
    println!("Welcome to the rust calculator!");
    println!("Please choose any of these options below:");
    println!("1: sum");
    println!("2: sub");
    println!("3: mul");
    println!("4: exit");

    let mut counter = 0u32;
    let mut op = String::new();

        while counter < 3 {
    standard_utils::get_stdin_method("your option: ", &mut op);
    let operation = standard_utils::str_to_i32(&mut op);

    if operation == 1 {
        println!("debug: you've choosen sum!");
        math::sm();
        loop_q::exit_loop();
    } else if operation == 2 {
        println!("debug: you've choosen sub!");
        math::sb();
        loop_q::exit_loop();
    } else if operation == 3 {
        println!("debug: you've choosen mul!");
        math::mul();
        loop_q::exit_loop();
    } else if operation == 4 {
        println!("debug: bye!");
        loop_q::exit();
    }
     else {
        println!("Error: you haven't choosen anything yet");
        counter = counter +1
  }
 }
}
