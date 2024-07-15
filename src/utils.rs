use crate::errors::DBError;
use serde_json::{self, Value};
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{
    path::Path,
};

pub fn trim_string(string: &mut str) -> &str {
    &string[1..string.len() - 1]
}

pub fn write_log(filepath: &Path, filename: &String, data: Value) -> Result<(), DBError> {
    let mut file = File::create(filepath.join(filename)).expect("Could not create file!");
    file.write_all(data.to_string().as_bytes())
        .expect("Cannot write to the file!");
    Ok(())
}

pub fn read_log(filepath: &Path, filename: &String)-> Result<Value, DBError>{
    let mut data = String::new();
    let mut f = File::open(filepath.join(filename));
    if let Ok(mut file) =  f {
        file.read_to_string(&mut data).map_err(|_err| DBError::Io);
        let data_dict: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
        Ok(data_dict)
    } else {
        Err(DBError::from_log_read())
    }  
}
 
    
//     Some(data_dict)
// }

pub fn delete_log(filepath: &Path, filename: &String) -> Result<(), DBError> {
    let _ = fs::remove_file(filepath.join(filename)).map_err(|_err| DBError::Io);
    Ok(())
}

pub fn count_files_in_dir(dir: &Path) -> io::Result<usize> {
    let count = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok()) // Filter out any erroneous entries
        .filter(|entry| entry.path().is_file())
        .count();
    Ok(count)
}
