mod lib;

use lib::{Gen, Person, Juridic};
use rand::{thread_rng, Rng};
use std::env;

fn generate_physical(n: u32) {
    for _i in 0..n {
        println!("{}", Person::generate());
    }
}

fn generate_juridic(n: u32) {
    for _i in 0..n {
        println!("{}", Juridic::generate());
    }
}

fn generate_misc(n: u32) {
    for _i in 0..n {
        let mut rng = thread_rng();
        let flag: u32 = rng.gen_range(0..2);
        if flag == 0 {
            println!("{}", Person::generate());
        } else if flag == 1 {
            println!("{}", Juridic::generate());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "--physical" {
        let n = args[2].parse::<u32>().unwrap();
        generate_physical(n);
    } else if args.len() == 3 && args[1] == "--juridic" {
        let n = args[2].parse::<u32>().unwrap();
        generate_juridic(n);
    } else if args.len() == 3 && args[1] == "--misc" {
        let n = args[2].parse::<u32>().unwrap();
        generate_misc(n);
    } else {
        println!("Generate a document numbers based on \"--physical\" or \"--juridic\" with \"n\" repetitions.");
    }
}
