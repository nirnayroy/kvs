// use serde_json::{self, json};
// use std::{
//     collections::{HashMap, HashSet},
//     path::Path,
// };
// pub mod errors;
// mod log;
// mod utils;
// use crate::errors::DBError;
// use crate::log::Log;
// use utils::trim_string;

// // impl KvsEngine{

// // }
// #[derive(Debug)]
// pub struct KvStore {
//     dict: HashMap<String, String>,
//     path: Box<Path>,
//     log: Log,
//     max_log: u64,
// }

// impl KvStore{
//     fn recreate_state_from_log(&mut self, uid: String) -> Result<(), DBError> {
//         let mut uid = uid;
//         let mut removed_or_set_later: HashSet<String> = HashSet::new();
//         while uid.to_string().parse::<u64>().unwrap() > 1 {
//             let log_dict = utils::read_log(&self.path, &uid.to_string());

//             let mut command_raw = log_dict["command"].to_string();
//             let command = trim_string(&mut command_raw);

//             let mut key_raw = log_dict["kv_pair"]["key"].to_string();
//             let key = trim_string(&mut key_raw);

//             if removed_or_set_later.contains(key) {
//                 uid = log_dict["previous"].to_string();
//             } else if command == "set" {
//                 self.dict.insert(key.to_string(), (*uid).to_string());
//                 removed_or_set_later.insert(key.to_string());
//                 uid = log_dict["previous"].to_string();
//             } else if command == "rm" {
//                 removed_or_set_later.insert(key.to_string());
//                 uid = log_dict["previous"].to_string();
//             } else {
//                 uid = log_dict["previous"].to_string();
//             }
//         }
//         Ok(())
//     }

//     pub fn get(&mut self, key: String) -> Result<Option<String>, DBError> {
//         self.log = self.log.load(&self.path).unwrap();
//         self.max_log = self.log.current;
//         self.recreate_state_from_log(self.max_log.to_string())?;

//         if self.dict.get(&key).map(|s| s.to_string()).is_some() {
//             let log_pointer = self.dict.get(&key).map(|s| s.to_string()).unwrap();
//             let log_dict = utils::read_log(&self.path, &log_pointer);
//             let mut value: String = log_dict["kv_pair"]["value"].to_string();
//             let value_slice = trim_string(&mut value);
//             println!("{}", value_slice);
//             Ok(Some(value_slice.to_string()))
//         } else {
//             println!("Key not found");
//             Ok(None)
//         }
//     }
//     pub fn set(&mut self, key: String, value: String) -> Result<Option<String>, DBError> {
//         self.log = self.log.load(&self.path).unwrap();
//         self.max_log = self.log.current;
//         self.recreate_state_from_log(self.log.current.to_string())?;
//         let log_pointer = (self.max_log + 1).to_string();
//         self.dict.insert((key.clone()).to_string(), log_pointer);
//         let kv_pair = json!(
//         {
//             "key": key,
//             "value": value,
//         });
//         self.log.append("set".to_string(), kv_pair, &self.path);
//         Ok(Some("".to_string()))
//     }

//     pub fn remove(&mut self, key: String) -> Result<Option<String>, DBError> {
//         self.log = self.log.load(&self.path).unwrap();
//         self.max_log = self.log.current;
//         self.recreate_state_from_log(self.log.current.to_string())?;
//         if self.dict.remove(&(key.clone()).to_string()).is_none() {
//             println!("Key not found");
//             Err(errors::DBError::no_key())
//         } else {
//             let kv_pair = json!(
//             {
//                 "key": key,

//             });
//             self.log.append("rm".to_string(), kv_pair, &self.path);
//             Ok(Some(key))
//         }
//     }

//     pub fn open(path: &Path) -> Result<KvStore, DBError> {
//         let log_init = Log::new();
//         Ok(KvStore {
//             // store: kv_store,
//             path: path.into(),
//             log: log_init.load(path).unwrap(),
//             dict: HashMap::new(),
//             max_log: 0,
//         })
//     }

//     pub fn compact_logs(&mut self) -> Result<(), DBError> {

//         println!("compaction triggered");
//         let mut removed_or_set_later: HashSet<String> = HashSet::new();
//         let current_log = &mut self.log.load(&self.path).unwrap();

//         while current_log.previous > 1 {
//             let mut previous_log = utils::read_log(&self.path, &current_log.previous.to_string());

//             let mut current_key_raw = current_log.log_dict["kv_pair"]["key"].clone().to_string();
//             let current_key = trim_string(&mut current_key_raw);

//             let mut previous_key_raw = previous_log["kv_pair"]["key"].clone().to_string();
//             let mut previous_key = trim_string(&mut previous_key_raw);

//             let mut previous_command_raw = previous_log["command"].clone().to_string();
//             let mut previous_command = trim_string(&mut previous_command_raw);

//             let mut current_command_raw = current_log.log_dict["command"].clone().to_string();
//             let current_command = trim_string(&mut current_command_raw);

//             while removed_or_set_later.contains(previous_key)
//                 & (previous_command == "set")
//                 & (current_log.previous > 2)
//             {
//                 // change to previous log and delete current
//                 current_log.log_dict["previous"] = previous_log["previous"].clone();
//                 let redundant_log = current_log.previous;
//                 current_log.previous = previous_log["previous"].as_u64().unwrap();
//                 utils::write_log(
//                     &self.path,
//                     &current_log.current.to_string(),
//                     current_log.log_dict.clone(),
//                 )?;
//                 utils::delete_log(&self.path, &redundant_log.to_string())?;
//                 previous_log = utils::read_log(&self.path, &current_log.previous.to_string());

//                 previous_key_raw = previous_log["kv_pair"]["key"].clone().to_string();
//                 previous_key = trim_string(&mut previous_key_raw);

//                 previous_command_raw = previous_log["command"].clone().to_string();
//                 previous_command = trim_string(&mut previous_command_raw);
//             }
//             if (current_command == "rm") | (current_command == "set") {
//                 removed_or_set_later.insert(current_key.to_string());
//             } else {
//                 utils::delete_log(&self.path, &current_log.current.to_string())?;
//             }
//             current_log.log_dict = previous_log.clone();
//             current_log.current = current_log.previous;
//             current_log.previous = previous_log["previous"].as_u64().unwrap();
//         }

//         println!("compaction completed");
//         Ok(())
//     }
// }

pub mod engine;
pub mod server;
pub mod errors;
pub mod log_pointer;
pub mod utils;
pub mod protocol;
pub mod client;