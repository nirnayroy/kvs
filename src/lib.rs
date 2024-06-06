use serde::de::value;
use serde::Serialize;
use serde_json::{self, json, Value};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::{collections::HashMap, path::Path};
use tempfile::TempDir;
use unique_id::sequence::SequenceGenerator;
use unique_id::Generator;

#[derive(Debug)]
pub enum DBError {
    Serialize(serde_json::Error),
    Io(io::Error),
    Log,
    NoKey
}

impl DBError {
    fn from_serialization(err: serde_json::Error) -> DBError {
        DBError::Serialize(err)
    }

    fn from_io(err: io::Error) -> DBError {
        DBError::Io(err)
    }

    fn from_log_read() -> DBError {
        DBError::Log
    }

    fn no_key() -> DBError {
        DBError::NoKey
    }
}



#[derive(Debug)]
pub struct Log {
    current: u64,
    previous: u64,
    value: Value,
}

impl Log {
    pub fn new() -> Log {
        // let gen = SequenceGenerator::default();
        Log {
            current: 1,
            previous: 0,
            value: json!(0),
        }
    }

    pub fn append(&mut self, entry: (String, Value), path: &Path) {
        self.previous = self.current;
        self.current = self.current + 1;
        self.value = self.create_entry(entry);
        self.write(path);
    }

    fn write(&self, path: &Path) {
        let mut file = File::create(path.join(self.current.to_string())).expect("Could not create file!");
        file.write_all(self.value.to_string().as_bytes())
            .expect("Cannot write to the file!");
    }

    fn create_entry(&self, args_list: (String, Value)) -> Value {
        json!({
            "previous": self.previous,
            "command": args_list.0,
            "value": args_list.1,
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
    pub fn load(&self, path: &Path) -> Result<Log, crate::io::Error>  {
        let unique_id = self.find_highest_numbered_file(path)?;
        // println!("{}", unique_id.clone().unwrap());
        match unique_id {
            None => Ok(Log::new()),
            Some(unique_id) => Log::get_log_from_dict(&unique_id, Log::read_log_dict(&unique_id, path)),
        } 
    }

    pub fn read_log_dict(unique_id: &String, path: &Path) -> Value {

        let mut data = String::new();
        let mut f =
            File::open(path.join(unique_id)).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read string");
        // println!("{}", data);
        let data_dict: serde_json::Value =
            serde_json::from_str(&data).expect("JSON was not well-formatted");
        data_dict
    }

    pub fn get_log_from_dict(unique_id: &String, data_dict: Value)-> Result<Log, crate::io::Error>{
        Ok(Log {
            current: unique_id.parse::<u64>().unwrap(),
            previous: data_dict["previous"].to_string().parse::<u64>().unwrap(),
            value: data_dict["value"].clone(),
        })
    }
    
}

#[derive(Debug)]
pub struct KvStore {
    // log: String,
    dict: HashMap<String, String>,
    path: Box<Path>,
    log: Log,
    max_log: u64,
}

impl KvStore {
    /// Create a new KvStore.

    // pub fn new() -> KvStore{
    // let kv_store = HashMap::new();
    //     KvStore {
    //         // log:
    //         store: kv_store,
    //     }
    // }

    fn recreate_state_from_log(&mut self, uid: String, target: &String) -> Result<(), DBError>{
        // println!("recursive uid {}", uid.clone());
        if uid.to_string().parse::<u64>().unwrap() > self.max_log {
            return Ok(())
        } else {
            let log_dict = Log::read_log_dict(&uid, &*self.path);
            let command = &log_dict["command"].to_string();
            // println!("log dict contents {}", log_dict["previous"]);
            // println!("log dict contents {}", log_dict["command"]);
            // println!("log dict contents {}", log_dict["value"]);
            // println!("log dict contents {}", log_dict);
            let short = &command[1..command.len()-1];

            match short {
                "set" => {

                    let key = &log_dict["value"]["key"].to_string();
                    let key_slice = &key[1..key.len()-1];

                    let value: &String = &log_dict["value"]["value"].to_string();
                    let value_slice = &value[1..value.len()-1];

                    if key_slice == target {
                        let key = &log_dict["value"]["key"].to_string();
                        let key_slice = &key[1..key.len()-1];
                        // println!("here {}", log_dict["value"]["key"]);
                        self.dict.insert(key_slice.to_string(), value_slice.to_string());
                        // println!("{}", self.dict.get("key1").unwrap());
                        self.recreate_state_from_log((log_dict["previous"].to_string().parse::<u64>().unwrap()+2).to_string(), target);
                        return Ok(())
                    } else {
                        self.recreate_state_from_log((log_dict["previous"].to_string().parse::<u64>().unwrap()+2).to_string(), target);
                        return Ok(())
                    }
                },
                "rm" => {
                    let key = &log_dict["value"]["key"].to_string();
                    let key_slice = &key[1..key.len()-1];
                    self.dict.remove(&key_slice.to_string());
                    self.recreate_state_from_log((log_dict["previous"].to_string().parse::<u64>().unwrap()+2).to_string(), target);
                    return Ok(())
                },
                "get" => {
                    self.recreate_state_from_log((log_dict["previous"].to_string().parse::<u64>().unwrap()+2).to_string(), target);
                    return Ok(())
                },
                _ => return Err(DBError::from_log_read()),
                }
            }
        }

    pub fn get(&mut self, key: String) -> Result<Option<String>, DBError> {
        // let dict = self.store;
        // let value = self.store.get(&key).map(|s| s.to_string()).unwrap();
        // print!("{:?}", &*self.path);

        self.log = self.log.load(&*self.path).unwrap();
        self.max_log = self.log.current;
        // println!("max log {}", self.max_log.clone());
        // println!("path {}", self.path.display());
        self.recreate_state_from_log("2".to_string(), &key)?;
        self.log.append(("get".to_string(), json!(
            {
                "key": key,
            })
        ), &*self.path);
        
        
        if self.dict.get(&key).map(|s| s.to_string()).is_some(){
            println!("{}", self.dict.get(&key).map(|s| s.to_string()).unwrap());
            Ok(self.dict.get(&key).map(|s| s.to_string()))
        } else {
            println!("Key not found");
            Ok(None)
        }

        // unimplemented!("unimplemented")
    }
    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>, DBError> {
        // self.store.insert((key.clone()).to_string(), value);
        // self.store.get(&key).map(|s| s.to_string());
        // let temp_dir = TempDir::new().expect("unable to create temporary working directory");

        // let dict = HashMap::new();
        // dict.insert((key.clone()).to_string(), value);
        // let entry= serde_json::to_string(&p).unwrap();
        // self.log.append(entry);

        // let file = std::fs::File::open(self.path).map_err(|err| DBError::from_io(err))?;
        // let file = std::io::BufWriter::new(file);
        // serde_json::to_writer(file, &dict).map_err(|err| DBError::from_serialization(err))?;
        self.log = self.log.load(&*self.path).unwrap();
        self.max_log = self.log.current;
        self.log.append(("set".to_string(), json!(
        {
            "key": key,
            "value": value,
        })
    ), &*self.path);
        self.dict.insert((key.clone()).to_string(), value);
        Ok(Some("".to_string()))
    }

    pub fn remove(&mut self, key: String) -> Result<Option<String>, DBError> {
        // self.store.remove(&(key.clone()).to_string());
        // let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        // let file = std::fs::File::open(temp_dir).map_err(|err| DBError::from_io(err))?;
        // let file = std::io::BufWriter::new(file);
        // serde_json::to_writer(file, &self.store).map_err(|err| DBError::from_serialization(err))?;
        // Ok(Some("".to_string()))
        self.log = self.log.load(&*self.path).unwrap();
        self.max_log = self.log.current;
        // println!("max log {}", self.max_log.clone());
        // println!("path {}", self.path.display());
        self.recreate_state_from_log("2".to_string(), &key)?;
        self.log.append(("rm".to_string(), json!(
            {
                "key": key,
            })
        ), &*self.path);

        if self.dict.remove(&(key.clone()).to_string()).is_none(){
            println!("Key not found");
            Err(DBError::no_key())
            // panic!()
        } else {
            Ok(Some(key))
        }
        
    }

    pub fn open(path: &Path) -> Result<KvStore, DBError> {

        // Parse the string of data into serde_json::Value.
        // let file = std::fs::File::open(path).map_err(|err| DBError::from_io(err))?;
        // let file = std::io::BufWriter::new(file);
        // serde_json::to_writer(file, &init_log).map_err(|err| DBError::from_serialization(err))?;
        // let v: serde_json::Value = serde_json::from_str(data).map_err(|err| DBError::from_serialization(err))?;
        // let db = KvStore{store:v};
        let log_init = Log::new();
        Ok(KvStore {
            // store: kv_store,
            path: path.into() ,
            log: log_init.load(path).unwrap(),
            dict: HashMap::new(),
            max_log: 0
        })
    }
}

// trait Foo{
//     fn get(&self) -> Self;
//     fn set(&self) -> Self;
//     fn remove(&self) -> Self;
// }
// impl Foo for KvStore{

// }
