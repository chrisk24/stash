use std::env;
use std::fs;
use std::path::PathBuf;

const SAVE_PATH: &str = "C:\\Users\\Public\\stash\\";


fn get_help_text() -> String {
    "Hello... \n 
        in case you need some help \n 
        Commands:\n
        Get Help \t\t stash help\n
        To Store \t\t stash send <filename>\n
        To Get \t\t stash ret <filename>\n
        To Delete \t\t stash del <filename>\n
        To List \t\t stash lst\n
        To Show \t\t stash cat <filename>".to_string()
}


fn list_files_stored() -> String {
    let paths: Vec<PathBuf> = fs::read_dir(SAVE_PATH).unwrap()
                                .map(|p| p.unwrap().path())
                                .collect();

    format!("{:?}", paths)
}


fn cat_file(arg: &str) -> String {
    let path = SAVE_PATH.to_string() + arg;
    let fl = fs::read_to_string(path).unwrap();
    fl
}

fn ret_file(arg: &str) -> String {
    let copy_to = arg.to_string();
    let copy_from = SAVE_PATH.to_string() + arg;
    fs::copy(&copy_from, &copy_to).unwrap();
    format!("{} -> {}", copy_from, copy_to)
}

fn stash_file(arg: &str) -> String {
    let copy_from = arg.to_string();
    let copy_to = SAVE_PATH.to_string() + arg;
    fs::copy(&copy_from, &copy_to).unwrap();
    format!("{} -> {}", copy_from, copy_to)
}


fn del_file(arg: &str) -> String {
    let path = SAVE_PATH.to_string() + arg;
    fs::remove_file(&path).unwrap();
    format!("Removed: {}",path)
}


//get help or list
fn process_two_args(args: Vec<String>) -> String {
    let flag: &str = args.get(1).unwrap(); 
    
    match flag {
        "lst" => list_files_stored(),
        _ => get_help_text(),
    }   
}


//get show, store, delete, retrieve
//requires: len(args) >= 3
fn process_three_args(args: Vec<String>) -> String {
    let flag: &str = args.get(1).unwrap();
    let arg: &str = args.get(2).unwrap();
    match flag {
        "cat" => cat_file(arg),
        "ret" => ret_file(arg),
        "send" => stash_file(arg),
        "del" => del_file(arg),
        _ => get_help_text(),
    }
}

//returns what will be printed to the user
fn process_args(args: Vec<String>) -> String {
    match args.len() {
        2 => process_two_args(args),
        3 => process_three_args(args),
        _ => get_help_text(),
    }
}





fn main() {
    println!("Hello, Stasher!");
    let args: Vec<String> = env::args().collect(); //accept command line args
    println!("{:?}", args); //debug print

    let result = process_args(args);
    println!("{}", result);
}
