use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::env;

const BEGIN : &'static str = "#include <stdlib.h>
#include <stdio.h>
#include \"std.h\"
static int SIZE_BUFF = 50;
int main() {";

const END : &'static str = "}";

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
    let mut variables : Vec<String> = Vec::new();
    let mut results : Vec<String> = Vec::new();

    let working_string = code_string.split(&[';', '\n'][..]);

    for x in working_string.clone() {
        println!("{}", &x);
    }    

    final_string.push_str(BEGIN);

    for current in working_string {
        
        if current == "" || current == "\n" || current == " " || current == "\t" {
            println!(" bad{}", &current);
            continue;
        }

        let current = current.trim();

        let mut splitting_str = String::from(current);
        let split_string = split_by_space(&mut splitting_str);

        fn is_empty(s: &str) -> bool {
            s != " " && s != "\n" && s != "\t" && s != ""
        }
        
        let t = split_string.iter().filter(|s| is_empty(s)).collect::<Vec<_>>();
        
        if t.len() == 0 {
            break;
        }

        let mut str_vec : Vec<String> = Vec::new();
        for x in t {
            str_vec.push(x.clone());
        }

        println!("{}", current);
        let tmp_str = str_vec[0].clone();
        let str_vec = str_vec.into_iter().filter(|s| is_empty(s)).collect::<Vec<_>>();
        match tmp_str.as_ref() {
            "dec" => {
                for arg in 1..str_vec.len() {
                     if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                        continue;
                    }

                    final_string.push_str(&format!("char* {} = malloc(sizeof(char) * SIZE_BUFF);\n", &str_vec[arg]));
                    
                    variables.push(str_vec[arg].clone());
                }
            },
            "res" => {

                let mut tmp = 1;

                loop {
                    if str_vec[tmp] == "" || str_vec[tmp] == "\n" || str_vec[tmp] == " " || str_vec[tmp] == "\t" {
                        tmp += 1;
                        continue;
                    }
                    else {
                        break;
                    }
                }
                
                if results.contains(&str_vec[tmp]) {
                    final_string.push_str(&format!("{} = ", &str_vec[tmp]));
                }
                else {
                    final_string.push_str(&format!("int {} = ", &str_vec[tmp]));
                    
                    results.push(str_vec[tmp].clone());
                }
                tmp += 1;

                loop {
                    if str_vec[tmp] == "" || str_vec[tmp] == "\n" || str_vec[tmp] == " " || str_vec[tmp] == "\t" {
                        tmp += 1;
                        continue;
                    }
                    else {
                        break;
                    }
                }

                final_string.push_str(&format!("{} (", str_vec[tmp]));

                for arg in (tmp+1)..str_vec.len() {

                    if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                        continue;
                    }

                    if str_vec[arg].starts_with("\"") {
                        final_string.push_str(&str_vec[arg]);
                    }
                    else {
                        final_string.push_str(&format!("{}", &str_vec[arg]));
                    }

                    if str_vec.len() -1 != arg {
                            final_string.push_str(",");
                    }
                    println!("{}", str_vec[arg]);
                }

                final_string.push_str(");\n");
            },
            "on" => {
                let mut tmp = 1;

                loop {
                    if str_vec[tmp] == "" || str_vec[tmp] == "\n" || str_vec[tmp] == " " || str_vec[tmp] == "\t" {
                        tmp += 1;
                        continue;
                    }
                    else {
                        break;
                    }
                }

                final_string.push_str(&format!("if ( {} ) {}\n", str_vec[tmp], "{"));
            },
            "while" => {
                let mut tmp = 1;

                loop {
                    if str_vec[tmp] == "" || str_vec[tmp] == "\n" || str_vec[tmp] == " " || str_vec[tmp] == "\t" {
                        tmp += 1;
                        continue;
                    }
                    else {
                        break;
                    }
                }

                final_string.push_str(&format!("while ( {} ) {}\n", str_vec[tmp], "{"));
            },
            "end" => {
                final_string.push_str("}\n");
            },
            _ => {
                final_string.push_str(&format!("{} (", tmp_str));
                println!("{}", str_vec.len());
                for arg in 1..str_vec.len() {

                     if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                        continue;
                    }

                    if str_vec[arg].starts_with("\"") {
                        final_string.push_str(&str_vec[arg]);
                    }
                    else {
                        final_string.push_str(&format!("{}", &str_vec[arg]));
                    }

                    if str_vec.len() -1 != arg {
                            final_string.push_str(",");
                    }
                    println!("{}", str_vec[arg]);
                }
                final_string.push_str(");\n");
            },
        }
    }

    for x in variables {
        final_string.push_str(&format!("free({});\n", x));
    }

    final_string.push_str(END);

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

fn split_by_space(inp : &mut String) -> Vec<String> {

    inp.push(' ');
    let mut fin : Vec<String> = Vec::new();

    let mut current = String::from("");

    let mut in_char = false;

    for x in inp.chars() {
        if (x == ' ' || x == '\n' || x == '\t') && !in_char {
            fin.push(current.clone());
            current = String::from("");
        }
        else if x == '"' {
            in_char = !in_char;
            current.push(x);
        }
        else {
            current.push(x);
        }
    }

    return fin;
}
