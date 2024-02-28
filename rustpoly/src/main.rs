#![allow(unused_imports)]

use poly_api::parameter_handles;
use std::env;
use std::fmt::Debug;
use std::path::{Path, PathBuf};


fn test_param_handles(){

    let current_directory = env::current_dir();
    let mut prog_dir: PathBuf = match current_directory {

        Err(why) => panic!("Could not get current working directory!\nReason: {}",why ),
        Ok(cd) => cd, 
        
    };

    prog_dir.push("src/program_files/file_paths.toml");

    let file_paths_table = parameter_handles::create_toml_table(prog_dir);
    parameter_handles::get_file_paths(file_paths_table, "program_files", "keys");

}


fn main(){

    println!("WORKING...");
    test_param_handles();

}