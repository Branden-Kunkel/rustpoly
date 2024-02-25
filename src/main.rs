#![allow(unused_imports)]

use poly_api::parameter_handles;

fn main(){

println!("Working!");

let table = parameter_handles::create_toml_table("/home/brandenk/Documents/programming/rust/rustpoly/src/program_files/file_paths.toml");
parameter_handles::get_file_paths(table);

}