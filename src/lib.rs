use serde_json::{self, json, Value};

use std::io;
use std::io::{Read, Write};
use std::slice::ChunksMut;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
pub mod errors;
mod log;
mod utils;
use crate::errors::DBError;
use crate::log::Log;
use utils::trim_string;

#[derive(Debug)]
pub struct KvStore {
    dict: HashMap<String, String>,
    path: Box<Path>,
    log: Log,
    max_log: u64,
}

impl KvStore {
    fn recreate_state_from_log(&mut self, uid: String, target: &String) -> Result<(), DBError> {
        if uid.to_string().parse::<u64>().unwrap() > self.max_log {
            Ok(())
        } else {
            let log_dict = utils::read_log(&self.path, &uid);
            let command = log_dict["command"].to_string();
            let short = trim_string(&command);

            match short {
                "set" => {
                    let key = log_dict["kv_pair"]["key"].to_string();
                    let key_slice = trim_string(&key);

                    let value = log_dict["kv_pair"]["value"].to_string();
                    let value_slice = trim_string(&value);

                    if key_slice == target {
                        self.dict.insert(key_slice.to_string(), uid.clone());
                        // println!("{}", self.dict.get("key1").unwrap());
                        let _ = self.recreate_state_from_log(
                            (uid.parse::<u64>().unwrap() + 1).to_string(),
                            target,
                        );
                        Ok(())
                    } else {
                        let _ = self.recreate_state_from_log(
                            (uid.parse::<u64>().unwrap() + 1).to_string(),
                            target,
                        );
                        Ok(())
                    }
                }
                "rm" => {
                    let key = log_dict["kv_pair"]["key"].to_string();
                    let key_slice = trim_string(&key);
                    self.dict.remove(&key_slice.to_string());
                    let _ = self.recreate_state_from_log(
                        (uid.parse::<u64>().unwrap() + 1).to_string(),
                        target,
                    );
                    Ok(())
                }
                "get" => {
                    let _ = self.recreate_state_from_log(
                        (uid.parse::<u64>().unwrap() + 1).to_string(),
                        target,
                    );
                    Ok(())
                }
                _ => Err(errors::DBError::from_log_read()),
            }
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>, DBError> {
        self.log = self.log.load(&self.path).unwrap();
        self.max_log = self.log.current;
        self.recreate_state_from_log("2".to_string(), &key)?;
        // self.log.append(
        //     (
        //         "get".to_string(),
        //         json!(
        //         {
        //             "key": key,
        //         }),
        //     ),
        // &self.path,
        // );

        if self.dict.get(&key).map(|s| s.to_string()).is_some() {
            let log_pointer = self.dict.get(&key).map(|s| s.to_string()).unwrap();
            let log_dict = utils::read_log(&self.path, &log_pointer);
            let value: String = log_dict["kv_pair"]["value"].to_string();
            let value_slice = trim_string(&value);
            println!("{}", value_slice.to_string());
            Ok(Some(value_slice.to_string()))
        } else {
            println!("Key not found");
            Ok(None)
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>, DBError> {
        self.log = self.log.load(&self.path).unwrap();
        self.max_log = self.log.current;
        // TODO
        // replace the "2" with a min log parameter for self
        // write function that updates min log if its deleted
        self.recreate_state_from_log("2".to_string(), &key)?;
        let log_pointer = (self.max_log + 1).to_string();
        self.dict.insert((key.clone()).to_string(), log_pointer);
        let kv_pair = json!(
        {
            "key": key,
            "value": value,
        });
        self.log.append("set".to_string(), kv_pair, &self.path);
        Ok(Some("".to_string()))
    }

    pub fn remove(&mut self, key: String) -> Result<Option<String>, DBError> {
        self.log = self.log.load(&self.path).unwrap();
        self.max_log = self.log.current;
        self.recreate_state_from_log("2".to_string(), &key)?;
        if self.dict.remove(&(key.clone()).to_string()).is_none() {
            println!("Key not found");
            Err(errors::DBError::no_key())
        } else {
            let kv_pair = json!(
            {
                "key": key,
            });
            self.log.append("rm".to_string(), kv_pair, &self.path);
            Ok(Some(key))
        }
    }

    pub fn open(path: &Path) -> Result<KvStore, DBError> {
        let log_init = Log::new();
        Ok(KvStore {
            // store: kv_store,
            path: path.into(),
            log: log_init.load(path).unwrap(),
            dict: HashMap::new(),
            max_log: 0,
        })
    }

    pub fn compact_logs(&mut self) -> Result<(), DBError> {
        //
        // removed_entries = [];
        // distinct_keys = [];
        //
        // if in
        let mut removed_or_set_later: HashSet<String> = HashSet::new();

        let mut current_log = self.log.load(&self.path).unwrap();

        while current_log.current > 1 {
            let mut previous_log = utils::read_log(&*self.path, &current_log.previous.to_string());
            while removed_or_set_later.contains(&previous_log["key"].to_string())
                | (previous_log["command"].to_string() == "get")
            {
                // change to previous log and delete current
                current_log.log_dict["previous"] = previous_log["previous"].clone();
                utils::write_log(
                    &self.path,
                    &current_log.current.to_string(),
                    current_log.log_dict.clone(),
                );
                utils::delete_log(&self.path, &current_log.previous.to_string());
                let mut previous_of_previous =
                    utils::read_log(&*self.path, &previous_log["previous"].to_string());
                previous_log = previous_of_previous;

                current_log.previous = previous_log["previous"].as_u64().unwrap();
            }
            if (current_log.log_dict["command"].to_string() == "rm")
                | (current_log.log_dict["command"].to_string() == "get")
            {
                removed_or_set_later.insert(current_log.log_dict["key"].to_string());
            } else {
                utils::delete_log(&self.path, &current_log.current.to_string());
            }
            current_log.log_dict = previous_log.clone();
            current_log.current = current_log.previous;
            current_log.previous = previous_log["previous"].as_u64().unwrap();
        }
        Ok(())
    }
}
