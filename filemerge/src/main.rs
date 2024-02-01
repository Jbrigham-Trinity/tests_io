//use std::io;
//use collections::vector;
use std::env;
use std::fs;

//takes two files, and returns a third with their contents.
//run from command line, with files supplied as arguments

fn main () {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    if args.len() < 3 {
        panic!("Didn't supply two files! Or didnt provide enough gibberish to fool me otherwise!")
    }

    let f1num = load_file_to_string(&args[1]);
    let f2num = load_file_to_string(&args[2]);
    let sum = (f1num + f2num).to_string(); //not using vars again, no address
    println!("{f2num} + {f1num}");

    println!("Making Result File (resfile) with sum of files provided (Really hard work!)");

    fs::write("resfile.txt", sum).expect("Unable to write file!");

    println!("Cool! Wrote file! Check it yourself!");
}

fn load_file_to_string (f: &String) -> i32 {
    let real = fs::read_to_string(f)
        .expect("Can't read file one?");
    let fnum = real.trim().parse::<i32>()
        .expect("Arg file isnt a number...!");
    fnum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let f1 = "src/test1.txt".to_string();
        let f2 = "src/test2.txt".to_string();
        let t1 = load_file_to_string(&f1);
        let t2 = load_file_to_string(&f2);
        assert_eq!(t1, 2);
        assert_eq!(t2,3);
    }
    
    
}