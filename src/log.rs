use crate::errors::{self, DBError};
use crate::utils;
use serde_json::{self, json, Value};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
#[derive(Debug)]
pub struct Log {
    pub current: u64,
    pub previous: u64,
    pub log_dict: Value,
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

impl Log {
    pub fn new() -> Log {
        Log {
            current: 1,
            previous: 0,
            log_dict: json!(0),
        }
    }

    pub fn append(&mut self, command: String, kv_pair: Value, path: &Path) {
        self.previous = self.current;
        self.current += 1;
        self.log_dict = self.create_entry(command, kv_pair);
        let _ = utils::write_log(path, &self.current.to_string(), self.log_dict.clone());
    }

    fn create_entry(&self, command: String, kv_pair: Value) -> Value {
        json!({
            "previous": self.previous,
            "command": command,
            "kv_pair": kv_pair,
        })
    }

    fn find_highest_numbered_file(&self, path: &Path) -> io::Result<Option<String>> {
        let mut highest_number: Option<u64> = None;
        let mut highest_file_name: Option<String> = None;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    if let Ok(number) = file_name.parse::<u64>() {
                        if highest_number.is_none() || number > highest_number.unwrap() {
                            highest_number = Some(number);
                            highest_file_name = Some(file_name.to_string());
                        }
                    }
                }
            }
        }

        Ok(highest_file_name)
    }
    pub fn load(&self, path: &Path) -> Result<Log, DBError> {
        let unique_id = self
            .find_highest_numbered_file(path)
            .map_err(|err| DBError::Io(err))?;
        match unique_id {
            None => Ok(Log::new()),
            Some(unique_id) => Ok(Log::get_log_from_dict(
                &unique_id,
                utils::read_log(path, &unique_id),
            )),
        }
    }

    pub fn get_log_from_dict(unique_id: &str, data_dict: Value) -> Log {
        Log {
            current: unique_id.parse::<u64>().unwrap(),
            previous: data_dict["previous"].to_string().parse::<u64>().unwrap(),
            log_dict: data_dict.clone(),
        }
    }
}
