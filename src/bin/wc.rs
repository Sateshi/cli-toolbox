use std::env;
use std::fs;

fn read_args(){
    let arguments = env::args();
    let stringified;
    if arguments.len() != 2 {
        println!("Please pass only one argument.");
        return;
    } else {
        stringified = fs::read_to_string(arguments.last().unwrap_or_else(|| "VIDE".to_owned()));
        match stringified {
            Ok(content) => {
                println!("{}",content);
            }
            Err(error) => {
                println!("Something went wrong : {}", error);
            }
        }
    }
}

fn main(){
    read_args();
}