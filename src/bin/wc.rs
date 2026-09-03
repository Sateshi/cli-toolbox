use std::env;
use std::fs;

fn count_words(argument: &String) -> usize{
    return argument.split_whitespace().count();
}

fn count_lines(argument: &String) -> usize{
    return argument.chars().filter(|c| *c == '\n').count();
}

fn count_bytes(argument: &String) -> usize{
    return argument.len();
}

fn read_args() -> Option<String>{
    let mut arguments = env::args();
    if arguments.len() == 2 {
        
        return arguments.nth(1);
    }
    return None;
}

fn main(){
    let arg = fs::read_to_string(read_args().unwrap()).unwrap();
    println!("{} {} {}",count_lines(&arg),count_words(&arg), count_bytes(&arg));
}