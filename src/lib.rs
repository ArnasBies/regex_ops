//to implement:
//Extract to new directory 
//Copy to new directory

use std::fs;
use regex::Regex;

pub struct Query{
    path: Option<std::path::PathBuf>,
    regex_expr: Option<String>,
    op: Operation,
}

enum Operation{
    Remove,
    Move(std::path::PathBuf),
    List,
    Help,
}

impl Operation{
    pub fn from_string(text: &str) -> Operation{
        match text{
            "Remove" | "remove" => return Operation::Remove,
            "Move" | "move" => return Operation::Move(std::path::PathBuf::new()),
            "List" | "list" => return Operation::List,
            "Help" | "help" => return Operation::Help,
            _ => panic!("Invalid Operation"),
        }
    }
}

impl Query{
    pub fn new(args: &Vec<String>) -> Query{
        //if args have at least two elements proceed as normal else contruct a help query where
        //elements aren't such as path or regex_expr ar not needed 
        //panic!("Please specify a path.\n(Query format: regex_ops operation regex_expression path)");
        let path: Option<std::path::PathBuf>;
        let regex_expr: Option<String>;

        let mut operation = match args.clone().get(1){
            Some(x) => Operation::from_string(x),
            None => panic!("Invalid operation, use 'regex_ops help' for help"),
        };

        match operation{
            //parse arguments for move command
            Operation::Move(_) => {
                regex_expr = match args.get(2){
                    Some(x) => Some(x.to_string()),
                    None => panic!("couldn't parse regex expression"),
                };

                path = match args.get(3){
                    Some(x) => {
                        let mut new_path = std::path::PathBuf::new();
                        new_path.push(x);
                        Some(new_path)
                    },
                    None => {
                        panic!("No path for Move operation 'regex_ops move regex_expression original_path move_path");
                    },
                };

                //get path where to move
                operation = match args.get(4){
                    Some(x) => {
                        let mut new_path = std::path::PathBuf::new();
                        new_path.push(x);
                        Operation::Move(new_path)
                    },
                        None => {
                        panic!("No second path for Move operation 'regex_ops move regex_expression original_path move_path");
                    },
                };
            },

            //set arguments to none if command is help
            Operation::Help => {
                path = None;
                regex_expr = None;
            },

            //get standard arguments for any other operation
            _ => {
                regex_expr = match args.get(2){
                    Some(x) => Some(x.to_string()),
                    None => panic!("couldn't parse regex expression"),
                };

                path = match args.get(3){
                    Some(x) => {
                        let mut new_path = std::path::PathBuf::new();
                        new_path.push(x);
                        Some(new_path)
                    },
                    None => {
                        panic!("Invalid path");
                    },
                };
            },
        }

        return Query { path, regex_expr, op: operation } 
    }

    pub fn execute(&self){
        match self.op{
            Operation::Move(_) => self.move_files(),
            Operation::List => self.list(),
            Operation::Help => self.help(),
            Operation::Remove => self.remove(),
        }
    }

    fn remove(&self){
        let paths = match fs::read_dir(self.path.clone().unwrap()){
            Ok(x) => x,
            Err(_) => panic!("failed to read items from directory"),
        };

        let expression = Regex::new(self.regex_expr.clone().unwrap().as_str()).unwrap();

        for path in paths{
            if let Ok(name) = path{
                let file_info = (name.path().clone(), name.file_name()); 

                if expression.is_match(file_info.1.clone().to_str().unwrap()){
                    match fs::remove_file(file_info.0){
                        Ok(_) => println!("successfully removed file: {}", file_info.1.clone().to_str().unwrap()),
                        Err(_) => println!("failed to remove file: {}", file_info.1.clone().to_str().unwrap()),
                    };
                }
            }
        }
    }

    fn list(&self){
        let paths = match fs::read_dir(self.path.clone().unwrap()){
            Ok(x) => x,
            Err(_) => panic!("failed to read items from directory"),
        };

        let expression = Regex::new(self.regex_expr.clone().unwrap().as_str()).unwrap();
        
        for path in paths{
            if let Some(name) = path.unwrap().file_name().to_str(){
                if expression.is_match(name){
                    println!("{name}");
                }
            }   
        }
    }

    fn move_files(&self){
        let paths = match fs::read_dir(self.path.clone().unwrap()){
            Ok(x) => x,
            Err(_) => panic!("failed to read items from directory"),
        };

        let expression = Regex::new(self.regex_expr.clone().unwrap().as_str()).unwrap();

        for path in paths{
            if let Ok(name) = path{
                let file_info = (name.path().clone(), name.file_name()); 

                if expression.is_match(file_info.1.clone().to_str().unwrap()){
                    match &self.op{
                        Operation::Move(x) => {
                            let mut move_path = x.clone();
                            move_path.push(file_info.1.clone().to_str().unwrap());

                            match std::fs::rename(file_info.0, move_path){
                                Ok(_) => println!("successfully moved file: {} to {}", file_info.1.clone().to_str().unwrap(), x.to_str().unwrap()),
                                Err(_) => println!("failed to move file: {} to {}", file_info.1.clone().to_str().unwrap(), x.to_str().unwrap()),
                            };
                        },
                        _ => println!("Invalid operation"),
                    }
                }
            }
        }
 
    }

    fn help(&self){
        println!("\tFormat: regex_ops command 'regex expression' path");
        println!("\tOn move command you have to specify an aditional path at the end in which the files will be moved to");
        println!("\tAvailable commands: Move, List, Help, Remove");
    }
}
