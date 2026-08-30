use crate::standard_utils;

use crate::utils::init_logic;
pub fn exit_loop() {
    
        println!("go back to main menu?: (y)es / (n)o");
            let mut exit = String::new();
    standard_utils::get_stdin(&mut exit);
standard_utils::UTF8_skip(&mut exit);

    let stdincomp: &str = &exit;


 if stdincomp == "y" {
        init_logic::quizz();
    } else if stdincomp == "n" {
        println!("bye!");
        std::process::exit(0);
     } else {
        println!("Error: you haven't choosen any option.");
        println!("this program will restart automatically...");
        init_logic::quizz();
     }
 }
pub fn exit() {
std::process::exit(0);
}