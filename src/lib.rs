// Lib for interacting with polygon.io

pub mod parameter_handles{

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn read_file(file_path: &str){

        let yaml_file_path = Path::new(file_path);

        let mut yaml_file: File = match File::open(&yaml_file_path) {
            
            Err(why) => panic!("Could not open file: {}\n{}", &yaml_file_path.display(), why),
            Ok(file) => file,
        };

        let mut yaml_string = String::new();

        let _file_to_str_buffer = yaml_file.read_to_string(&mut yaml_string);

        println!("{}", yaml_string)

    }


}

