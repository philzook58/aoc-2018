use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
	let filename = "test.txt";
    println!("Hello, world!");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // println!("{}",contents);
    let v = contents.lines(); //  .split("\n");
    // v.what();
    let q = v.map(|n : &str| -> i32 {n.parse().unwrap()});
    //q.what();
    // let acc: i32 = q.sum();
    let infq = q.cycle();

    /*
    let acc : Vec<i32> = q.scan(0, |state, n| {
    	*state = *state + n;
        Some(*state)	
    } ).collect();

    let 
    let mut seen = HashSet::new();
    let mut winner = 3;
    for n in acc {
    	//println!("{:?}",winner);
    	if seen.contains(&n) {
    		println!("{:?}",winner);
    		winner = n;
    		break;
    	}
    	else{
    		seen.insert(n);
    		println!("{:?}",seen);
    	}

    }
    */
    let mut seen = HashSet::new();
    let mut state = 0;
    for n in infq{
    	if seen.contains(&state) {
    		//println!("{:?}",winner);
    		//winner = n;
    		break;
    	}
    	else{
    		seen.insert(state);
    		state += n;
    		println!("{:?}",state);
    		
    	}
    }
    println!("{:?}",state);
    /*
    let mut acc = 0;
    for num in v {
    	println!("{}", num);
    	let n: i32 = num.parse().unwrap();
    	acc += n;
    }*/
    //println!("{:?}",winner);
    //println!("{:?}",seen);
}
