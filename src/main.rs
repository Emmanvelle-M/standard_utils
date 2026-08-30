mod lib { pub mod standard; }
mod utils {
pub mod loop_q;
pub mod init_logic;
pub mod math;

}
use crate::lib::standard::standard as standard_utils;


fn main() {
utils::init_logic::quizz();
// let mut input_skip = String::new();

// standard_utils::get_stdin(&mut input_skip);
//standard_utils::UTF8_skip(&mut input_skip);
// println!("{}", input_skip);
}