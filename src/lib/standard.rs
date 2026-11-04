#![allow(warnings)]

pub mod standard {
    use std::fmt::Debug;
    use std::str::FromStr;
    use std::io;
    use std::io::{Write, stdin, stdout};
    pub fn get_stdin(get: &mut String) {
    let mut stdinp = String::new();
io::stdin().read_line(&mut stdinp).expect("failed to readline");
    *get = stdinp;
  }

  pub fn get_stdin_method(wait_string: &str, get: &mut String) {
    print!("{}", wait_string);
    stdout().flush().unwrap();
    let mut stdinp = String::new();
    io::stdin().read_line(&mut stdinp).unwrap();
    stdinp.pop();
    *get = stdinp;
  }

  pub fn UTF8_skip(UTF8_WAIT: &mut String) {
    let UTF8_SKIP = UTF8_WAIT
    .replace(['\n', '\t'], " ").trim().split(' ')
    .filter(|short| !short.is_empty()).collect::<Vec<_>>()
    .join(" ");
    *UTF8_WAIT = UTF8_SKIP.clone();
  }

pub fn get_stdin_str(get: &mut String) -> String {
  io::stdin().read_line(&mut *get).expect("failed to readline");
  get.replace(['\n', '\t'], " ").trim().split(' ').filter(|short| !short.is_empty()).collect::<Vec<_>>().join(" ")
  }

pub fn get_stdin_to_u8_method(get: &mut String) -> u8 {
  io::stdin().read_line(&mut *get).expect("failed to readline");
  get.trim().parse::<u8>().expect("unkown")
  }

pub fn get_stdin_to_u16_method(get: &mut String) -> u16 {
  io::stdin().read_line(&mut *get).expect("failed to readline");
  get.trim().parse::<u16>().expect("unkown")
  }

pub fn get_stdin_to_u32_method(get: &mut String) -> u32 {
  io::stdin().read_line(&mut *get).expect("failed to readline");
  get.trim().parse::<u32>().expect("unkown")
  }

pub fn get_stdin_to_u64_method(get: &mut String) -> u64 {
  io::stdin().read_line(&mut *get).expect("failed to readline");
  get.trim().parse::<u64>().expect("unkown")
  }

/* Experimental implementation Unknown limits :)
pub fn get_stdin_to_u128_method(get: &mut String) -> u128 {
  io::stdin().read_line(&mut *get).expect("failed to readline");
  get.trim().parse::<u128>().expect("unkown")
  }
*/

  pub fn str_to_i8(var: &mut String) -> i8 {
    var.trim().parse::<i8>().expect("unkown") 
  }
  pub fn str_to_i16(var: &mut String) -> i16 {
    var.trim().parse::<i16>().expect("unkown") 
  }
  pub fn str_to_i32(var: &mut String) -> i32 {
    var.trim().parse::<i32>().expect("unkown")
  }
  pub fn str_to_i64(var: &mut String) -> i64 {
    var.trim().parse::<i64>().expect("unkown")
  }
  pub fn str_to_i128(var: &mut String) -> i128 {
    var.trim().parse::<i128>().expect("unkown")
  }
}

/* This library is under The GNU Licence */
