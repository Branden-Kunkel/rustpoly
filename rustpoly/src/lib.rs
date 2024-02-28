// Lib for interacting with polygon.io

#![allow(unused_imports)] //Remove for release build
#![allow(dead_code)]

pub mod parameter_handles{

    use std::borrow::Borrow;
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Debug, Display};
    use std::fs::{read_to_string, File};
    use std::io::{BufRead, BufReader, Read};
    use std::ops::Index;
    use std::path::{self, Path, PathBuf};
    use std::str::FromStr;
    use std::vec;
    use toml::{Table, Value};
    use toml::map::Map;


    pub fn create_toml_table(file_path: PathBuf) -> Table{
 

        let toml_file_path = file_path.as_path();
        let mut toml_str_buffer = String::new();

        let mut toml_file: File = match File::open(&toml_file_path) {

            Err(why) => panic!("Could not open file: {}!\nReason: {}", &toml_file_path.display(), why),
            Ok(file) => file,
            
        };

        let mut _read_toml_to_string = match toml_file.read_to_string(&mut toml_str_buffer) {

            Err(why) => panic!("File: {} was opened but could not be read!\nReason: {}", &toml_file_path.display(), why),
            Ok(string) => string,
            
        }; 

        let toml_table: Table = match toml_str_buffer.parse() {

            Err(why) => panic!("Could not read File: {} to Table!\nReason: {}", &toml_file_path.display(), why),
            Ok(table) => table,
            
        };

        toml_table
        

        }


    pub fn get_file_paths(paths_table: Table, header: &str, key: &str) { 


        for header_entry in paths_table.values(){

            let is_val_there: bool = paths_table[header].eq(header_entry); //panics when key is not in index -> need a work around

            if is_val_there {
                
                let keys_table = match Table::try_from(header_entry) {
                    
                    Err(why) => panic!("No: {}", why),
                    Ok(table) => table,

                };
                
                for path_key in keys_table.values() {

                    let is_key_there: bool = keys_table[key].eq(path_key); //panics when key is not in index -> need a work around

                    if is_key_there{

                        println!("{}", keys_table);
                        println!("{}", path_key);

                    } 

                }

            }
            

        }

        }
    }
        