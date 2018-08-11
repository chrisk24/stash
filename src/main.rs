extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate dirs;


use std::env;
use std::fs;
use std::path::PathBuf;


#[derive(Deserialize)]
struct Config {
    save_path: String
}


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

fn list_files_stored(args: &Vec<String>, save_path: &PathBuf) -> String {
    let paths: Vec<PathBuf> = fs::read_dir(save_path).unwrap()
                                .map(|p| p.unwrap().path())
                                .collect();

    format!("{:?}", paths)
}


fn cat_file(args: &Vec<String>, save_path: &PathBuf) -> String {
    
    let filename = args.get(2).unwrap();

    let path = save_path.join(&filename);
    
    //println!("{}", &path);
    
    let fl = fs::read_to_string(&path).unwrap();
    
    fl
}

fn ret_file(args: &Vec<String>, save_path: &PathBuf) -> String {
    let filename = args.get(2).unwrap();

    //let copy_to = (&filename).to_string();
    let copy_from = save_path.join(&filename);
    fs::copy(&copy_from, &filename).unwrap();
    format!("{:?} -> {}", &copy_from, &filename)
}

fn stash_file(args: &Vec<String>, save_path: &PathBuf) -> String {
    let filename = args.get(2).unwrap();

    //let copy_from = (&filename).to_string();
    let copy_to = save_path.join(&filename);
    fs::copy(&filename, &copy_to).unwrap();
    format!("{} -> {:?}", &filename, &copy_to)
}


fn del_file(args: &Vec<String>, save_path: &PathBuf) -> String {
    let filename = args.get(2).unwrap();
    
    let path = save_path.join(&filename);
    fs::remove_file(&path).unwrap();
    format!("Removed: {:?}",path)
}


//returns what will be printed to the user
fn process_args(args: &Vec<String>, save_path: &PathBuf) -> String {
    let arg: &str = args.get(1).unwrap();

    match arg {
        "cat" => cat_file(&args, save_path),
        "ret" => ret_file(&args, save_path),
        "send" => stash_file(&args, save_path),
        "del" => del_file(&args, save_path),
        "lst" => list_files_stored(&args, save_path),
        _ => get_help_text()
    }
}

//searches for the config file in the home directory,
//the default directory
fn get_config(config_name:  &str) -> Config {
    println!("{}",config_name);

    let config_path_base = dirs::home_dir().unwrap();
    
    println!("{:?}", &config_path_base);

    let config_path = config_path_base.join("stash").join(config_name);
    
    println!("{:?}", &config_path);

    let config_str = fs::read_to_string(&config_path).unwrap();

    println!("{:?}", &config_str);

    let config: Config = toml::from_str(&config_str).unwrap();
   
    config
}


fn main() {
    let args: Vec<String> = env::args().collect(); //accept command line args
    println!("{:?}", args); //debug print
    let config = get_config("Stash.toml");
    println!("Config: {:?}", &config.save_path);
    let save_path = PathBuf::from(&config.save_path);
    let result = if (args.len() > 1) 
                    {process_args(&args, &save_path)}
                else 
                    {get_help_text()};

    println!("{}", result);
}
