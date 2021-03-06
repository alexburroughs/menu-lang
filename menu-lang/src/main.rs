use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;

const BEGIN : &'static str = "#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include \"std.h\"
#include \"arraylist.h\"
static int SIZE_BUFF = 50;
int main(int argc, char** argv) {";

const END : &'static str = "}";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("please provide a input filename and output filename as arguments");
    }

    let in_filename = &args[1];
    let out_filename = &args[2];

    let code_opt = load_file(&in_filename);
    let mut code_string : String;

    match code_opt {
        Some(x) => code_string = x,
        None => panic!("error unwrapping file"),
    }
	let mut code_string = delete_comments(&code_string);
    let res = parse_file(&mut code_string);

    out_file(&out_filename, &res);
}

fn parse_file(code_string : &mut String) -> String {

    let mut final_string = String::new();
    let mut variables : Vec<String> = Vec::new();
    let mut results : Vec<String> = Vec::new();
    let mut pointers : Vec<String> = Vec::new();

    let mut is_res = false;
	let mut in_function = false;

    let mut tmp_str = code_string.clone();

    let mut lines : Vec<Statement> = Vec::new();

    for mut x in split_line(&mut tmp_str) {
       lines.push(get_line_tokens(&mut x));
    }

    let working_string = code_string.split(&[';', '\n'][..]).collect::<Vec<_>>(); 

    final_string.push_str(BEGIN);

    for current in working_string {
        
        //println!("current: {}", current);
        if current == "" || current == "\n" || current == " " || current == "\t" {
            println!("bad: {}", &current);
            continue;
        }

        let current = current.trim();
        println!("current2: {}", current);
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

        let tmp_str = str_vec[0].clone();
        let mut str_vec = str_vec.into_iter().filter(|s| is_empty(s)).collect::<Vec<_>>();
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
            "list" => {
                for arg in 1..str_vec.len() {
                     if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                        continue;
                    }

                    final_string.push_str(&format!("ArrayList* {} = list_new(SIZE_BUFF);\n", &str_vec[arg]));
                    
                    variables.push(str_vec[arg].clone());
                }
            },
            "res" => {

                let mut tmp = 1;

                loop {
                    if str_vec[tmp] == "" || str_vec[tmp] == "\n" || str_vec[tmp] == " " || str_vec[tmp] == "\t" {
                        tmp += 1;
                    }
                    else {
                        break;
                    }
                }
                

                if str_vec[tmp] == "!" {
                    is_res = true;
                    if results.contains(&String::from("dec")) {
                        final_string.push_str("dec = ");
                    }

                    else {
                        final_string.push_str("int dec = ");
                        
                        results.push(str_vec[tmp].clone());
                    }
                }

                else if results.contains(&str_vec[tmp]) {
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
                    }
                    else {
                        break;
                    }
                }
                if str_vec[tmp] == ">" {
                    final_string.push_str("if ( dec ) {");
                }
                else {
                    final_string.push_str(&format!("if ( {} ) {}\n", str_vec[tmp], "{")); 
                }
            },
            "while" => {
                let mut tmp = 1;

                loop {
                    if str_vec[tmp] == "" || str_vec[tmp] == "\n" || str_vec[tmp] == " " || str_vec[tmp] == "\t" {
                        tmp += 1;
                    }
                    else {
                        break;
                    }
                }

                final_string.push_str(&format!("while ( {} ) {}\n", str_vec[tmp], "{"));
            },
            "def" => {
                let mut tmp = 1;
                let mut to_be_added : String = String::from("");

                if str_vec[tmp] == "res" {
                    tmp += 1;
                    to_be_added.push_str("int ");
                }
                to_be_added.push_str(&format!(" (* {} ) (", &str_vec[tmp]));
                pointers.push(str_vec[tmp].clone());
                tmp += 1;

                for arg in tmp..str_vec.len() {
                     if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                        continue;
                    }
                    str_vec[arg] = str_vec[arg].trim().to_string();
                    match str_vec[arg].as_ref() {
                        "str" => {
                            to_be_added.push_str("char*");
                            if str_vec[arg+1] != "->" {
                                to_be_added.push(',');
                            }
                        },

                        "res" => {
                            to_be_added.push_str("int");
                            if str_vec[arg+1] != "->" {
                                to_be_added.push(',');
                            }
                        },
                        "list" => {
                            to_be_added.push_str("list*");
                            if str_vec[arg+1] != "->" {
                                to_be_added.push(',');
                            }
                        },
                        "->" => {
                            to_be_added.push_str(") ");
                        },
                        _ => {
                            to_be_added.push_str(&format!("= {};\n", str_vec[arg]));
                        },
                    }
                
                }

                final_string.push_str(&to_be_added);
            },
			"collect" => {
				if !in_function {
					let mut current_ind = 1;
					for arg in 1..str_vec.len() {

                      	if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
                         	continue;
                     	}

						final_string.push_str(&format!("char* {}  = malloc(sizeof(char) * SIZE_BUFF);\n", &str_vec[arg]));
						final_string.push_str(&format!("strcpy({}, argv[{}]);\n", &str_vec[arg], &current_ind));
                    	variables.push(str_vec[arg].clone());
		
						current_ind += 1;
					}
				}
			},
			"expect" => {
				if !in_function {
					let mut to_be_added : String = String::from("");

					for arg in 1..str_vec.len() {
						if str_vec[arg] == "" || str_vec[arg] == "\n" || str_vec[arg] == " " || str_vec[arg] == "\t" {
							continue;
						}

						to_be_added.push_str(&format!("{}, ", str_vec[arg]));
					}	
					
					final_string.push_str(&format!("expect( {} argc);\n", to_be_added));
				}
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

/* load_file
 *
 * input:   Path to read from
 * purpose: Read the contents of the file
 * output:  String of the file contents
 */
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

/* out_file
 *
 * input:   Path to write to, contents to write
 * purpose: Write the contents to the file
 * output:  None
 */
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

/* split_by_space
 *
 * input:   String to split on
 * purpose: Split a string by spaces ignoring doubles
 * output:  Vector of strings
 */
fn split_by_space(inp : &mut String) -> Vec<String> {

    inp.push(' ');
    let mut fin : Vec<String> = Vec::new();

    let mut current = String::from("");

    let mut in_char = false;

    for x in inp.chars() {
        if (x == ' ' || x == '\n' || x == '\t') && !in_char{
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

/* split_line
 *
 * input:   The string to be split
 * purpose: Split a string into a vector of lines
 * output:  Vector of lines
 */
fn split_line(inp : &mut String) -> Vec<String> {

    let mut final_vec : Vec<String> = Vec::new();

    let mut current_str = String::from("");
    let mut is_line = false;

    for x in inp.chars() {
        if (x == '\n' || x == ';') && !is_line{
            
            final_vec.push(current_str);
            current_str = String::from("");
            is_line = true;
        }
        else if x != '\n' && x != ';' {
            current_str.push(x);
            is_line = false;
        }
    }

    final_vec
}

fn delete_comments(inp : &String) -> String {

	let mut ret_str = String::from("");
	let mut in_comment = false;
	let mut in_string = false;

	for x in inp.chars() {
		if x == '"' {
			in_string = !in_string;
		}
		if x == '*' && !in_string {
			in_comment = !in_comment;
			continue;
		}

		if !in_comment {
			ret_str.push(x);
		}
	}

	return ret_str;
}

/* get_line_tokens
 *
 * input:   String of the current line to be converted
 * purpose: Produce a statement object from a line
 * output:  Statement object containing the strings in 
 *          the line and the statement type
 */
fn get_line_tokens(line : &mut String) -> Statement {

    let mut stmt : Statement = Statement::default();

    stmt.tokens = split_by_space(line);

    stmt.stmt_type =
        match stmt.tokens
        .get(0)
        .expect("Error unwrapping tokens")
        .as_ref() {
            "res" => StatementType::Res,
            "dec" => StatementType::Dec,
            "def" => StatementType::Def,
            "list" => StatementType::List,
            "on" => StatementType::On,
            "while" => StatementType::While,
            "end" => StatementType::End,
			"collect" => StatementType::Collect,
			"expect" => StatementType::Expect,
            _ => StatementType::Call
        };

    stmt
}


struct Statement {
    tokens : Vec<String>,
    stmt_type : StatementType,
}

impl Default for Statement {
    fn default() -> Statement {
        Statement {
            tokens: Vec::new(),
            stmt_type: StatementType::Res,
        }
    }
}

/*
 * The statements command to add and then execute
 */
enum StatementType {
    Res,
    Dec,
    Def,
    List,
    On,
    While,
    End,
    Call,
	Proc,
	Func,
	Collect,
	Expect
}
