use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let filename = "data.txt";
    println!("Hello, world!");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // println!("{}",contents);
    let v = contents.lines(); //  .split("\n");
    // v.what();
    //let q = v.map(|n : &str| -> i32 {n.parse().unwrap()});
    let num_a : Vec<usize> = v.map(|s : &str| s.chars().filter(|c| c == &'a').count()).collect();
    //q.what();
    // let acc: i32 = q.sum();
    //let infq = q.cycle();
    println!("{:?}", num_a);
}
