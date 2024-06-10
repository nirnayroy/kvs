use serde_json::{self, json, Value};
use std::io::{Read, Write};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
use std::fs::File;
use std::fs;
use crate::errors::DBError;

pub fn trim_string(string: &str) -> &str{
    &string[1..string.len() - 1]
}

pub fn write_log(filepath:&Path, filename: &String, data: Value)->Result<(), DBError>{
    let mut file =
            File::create(filepath.join(filename)).expect("Could not create file!");
    let data_string = data.to_string();
    file.write_all(data.to_string().as_bytes())
            .expect("Cannot write to the file!");
    Ok(())
}

pub fn read_log(filepath:&Path, filename: &String)-> Value{
    let mut data = String::new();
    let mut f = File::open(filepath.join(filename)).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    let data_dict: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    data_dict
}

pub fn delete_log(filepath:&Path, filename: &String)->Result<(), DBError>{
    fs::remove_file(filepath.join(filename)).map_err(|err| DBError::Io(err));
    Ok(())
}