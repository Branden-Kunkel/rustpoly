// Lib for interacting with polygon.io

#![allow(unused_imports)] //Remove for release build
#![allow(dead_code)]

pub mod parameter_handles{

    use std::any::Any;
    use std::fmt::{Debug, Display};
    use std::fs::{read_to_string, File};
    use std::io::{BufRead, BufReader, Read};
    use std::ops::Index;
    use std::path::{self, Path};
    use std::vec;
    use toml::{Table, Value};
    use toml::map::Map;


    pub fn create_toml_table(file_path: &str) -> Table{
 

        let toml_file_path = Path::new(&file_path);
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


    pub fn get_file_paths(paths_table: Table) { 

        for header in paths_table.keys(){

                let header_str: &str = "program_files";

                if header == header_str {

                    println!("{}", header);

                }
                
            }

        }
    }

        

        
 

    




