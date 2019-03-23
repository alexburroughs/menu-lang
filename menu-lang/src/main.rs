use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use std::env;

const begin : &'static str = "#include <stdlib>
#include <stdio>
                                int main() {";

const end : &'static str = "}";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("please provide a input filename and output filename as arguments");
    }

    let in_filename = &args[1];
    let out_filename = &args[2];

    let code_opt = load_file(&in_filename);
    let code_string : String;

    match code_opt {
        Some(x) => code_string = x,
        None => panic!("error unwrapping file"),
    }

    let res = parse_file(&code_string);

    out_file(&out_filename, &res);
}

fn parse_file(code_string : &String) -> String {

    let mut final_string = String::new();
    let mut variables : HashMap<String,String> = HashMap::new();

    let working_string = code_string.split(";");

    final_string.push_str(begin);

    for current in working_string {
        
        if current == "" || current == "\n" || current == " " || current == "\t" {
            break;
        }
        let split_string = current.split(" ");

        fn is_empty(s: &str) -> bool {
            s != " " && s != "\n" && s != "\t" && s != ""
        }
        
        let t = split_string.filter(|s| is_empty(s));
        
        let mut str_vec : Vec<String> = Vec::new();

        for x in t {
            str_vec.push(String::from(x).clone());
        }

        println!("here");
        let tmp_str = str_vec[0].clone();

        match tmp_str.as_ref() {
            "out" => {
                final_string.push_str("printf(\"");
                for arg in 1..str_vec.len() {
                    if str_vec[arg].starts_with('"') {
                        let tmp = str_vec[arg].replace('"', "");
                        final_string.push_str(&tmp);
                    }
                    else if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                        continue;
                    }

                    if str_vec.len() != arg {
                            final_string.push_str(",");
                    }
                }
                final_string.push_str("\");");
            },
            _ => {},
        }
    }

    final_string.push_str(end);

    return final_string;
}

fn load_file(path : &String) -> Option<String> {

    let file_path = File::open(path);
    let mut file : File;
    let mut map_string = String::new();

    match file_path {
        Ok(res) => {file = res},
        Err(_) => return None
    }

    return match file.read_to_string(&mut map_string) {
        Ok(_) => {Some(map_string)},
        Err(_) => None
    }
}

fn out_file(path : &String, contents : &String) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(_) => {
            panic!("couldn't write to file");
        },
        Ok(_) => println!("compilation successful"),
    }
}
