use std::env;

use rand_passwd_rs::rand_gen_pass;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    assert!(args.len() == 2, "expected only 1 arg");
    let length = args[1].parse::<usize>().expect("argument must be a number");
    println!("{}", rand_gen_pass(length));
}
