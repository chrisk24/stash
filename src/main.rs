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

fn list_files_stored(args: &Vec<String>) -> String {
    let paths: Vec<PathBuf> = fs::read_dir(SAVE_PATH).unwrap()
                                .map(|p| p.unwrap().path())
                                .collect();

    format!("{:?}", paths)
}


fn cat_file(args: &Vec<String>) -> String {
    
    let filename = args.get(2).unwrap();

    let path = SAVE_PATH.to_string() + &filename;
    
    //println!("{}", &path);
    
    let fl = fs::read_to_string(&path).unwrap();
    
    fl
}

fn ret_file(args: &Vec<String>) -> String {
    let filename = args.get(2).unwrap();

    //let copy_to = (&filename).to_string();
    let copy_from = SAVE_PATH.to_string() + &filename;
    fs::copy(&copy_from, &filename).unwrap();
    format!("{} -> {}", &copy_from, &filename)
}

fn stash_file(args: &Vec<String>) -> String {
    let filename = args.get(2).unwrap();

    //let copy_from = (&filename).to_string();
    let copy_to = SAVE_PATH.to_string() + &filename;
    fs::copy(&filename, &copy_to).unwrap();
    format!("{} -> {}", &filename, &copy_to)
}


fn del_file(args: &Vec<String>) -> String {
    let filename = args.get(2).unwrap();
    
    let path = SAVE_PATH.to_string() + &filename;
    fs::remove_file(&path).unwrap();
    format!("Removed: {}",path)
}


//returns what will be printed to the user
fn process_args(args: &Vec<String>) -> String {
    let arg: &str = args.get(1).unwrap();

    match arg {
        "cat" => cat_file(&args),
        "ret" => ret_file(&args),
        "send" => stash_file(&args),
        "del" => del_file(&args),
        "lst" => list_files_stored(&args),
        _ => get_help_text()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect(); //accept command line args
    println!("{:?}", args); //debug print

    let result = if (args.len() > 1) 
                    {process_args(&args)}
                else 
                    {get_help_text()};

    println!("{}", result);
}
