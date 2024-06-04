use serde::de::value;
use serde::Serialize;
use serde_json::{self, json, Value};
use std::io;
use std::{collections::HashMap, path::Path};
use tempfile::TempDir;
use unique_id::Generator;
use unique_id::sequence::SequenceGenerator;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum DBError {
    Serialize(serde_json::Error),
    Io(io::Error),
}

impl DBError {
    fn from_serialization(err: serde_json::Error) -> DBError {
        DBError::Serialize(err)
    }

    fn from_io(err: io::Error) -> DBError {
        DBError::Io(err)
    }
}

#[derive(Debug)]
pub struct KvStore {
    // log: String,
    // store: HashMap<T, T>,
    // path: Path,
    log: Log,
}

#[derive(Debug)]
pub struct Log {
    current: i64,
    previous: i64,
    value: Value,
    rand_gen: SequenceGenerator,
}


impl Log {
    pub fn new() -> Log {
        let gen = SequenceGenerator::default();
        Log{
            current: gen.next_id(),
            previous: 0,
            value: json!(0),
            rand_gen: gen,
        }
    }

    pub fn append(&mut self, entry: Vec<String>){
        self.previous = self.current;
        self.current = self.rand_gen.next_id();
        self.value = self.create_entry(entry);
        self.write()
    }
    
    fn write(&self){
        let mut file = File::create(self.current.to_string()).expect("Could not create file!");
    
        file.write_all(self.value.to_string().as_bytes())
            .expect("Cannot write to the file!");
    }
    fn create_entry(&self, args_list: Vec<String>) -> Value{
        json!({
            "previous": self.previous,
            "command": args_list[0],
            "key": args_list[1],
            "value": args_list[2],
        })

    }

    // pub fn read(&self, path) -> Result<(), DBError>{

    // }

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

    pub fn get(&self, key: String) -> Result<Option<String>, DBError> {
        // let dict = self.store;
        // let value = self.store.get(&key).map(|s| s.to_string()).unwrap();
        // print!("{:?}", value);
        Ok(Some("".to_string())) 
        // unimplemented!("unimplemented")
    }
    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>, DBError>{
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
        Ok(Some("".to_string()))
    }

    pub fn remove(&mut self, key: String) -> Result<Option<String>, DBError> {
        // self.store.remove(&(key.clone()).to_string());
        // let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        // let file = std::fs::File::open(temp_dir).map_err(|err| DBError::from_io(err))?;
        // let file = std::io::BufWriter::new(file);
        // serde_json::to_writer(file, &self.store).map_err(|err| DBError::from_serialization(err))?;
        Ok(Some("".to_string()))
    }

    pub fn open(path: &Path) -> Result<KvStore, DBError> {
    //     // Some JSON input data as a &str. Maybe this comes from the user.
    //     //     let data = r#"
    //     // {
    //     //     "name": "John Doe",
    //     //     "age": 43,
    //     //     "phones": [
    //     //         "+44 1234567",
    //     //         "+44 2345678"
    //     //     ]
    //     // }"#;
    //     let kv_store = Table {
    //         name: "Default".to_owned(),
    //         data: HashMap::new(),
    //     };
        // Parse the string of data into serde_json::Value.
        let init_log = Log::new();
        let file = std::fs::File::open(path).map_err(|err| DBError::from_io(err))?;
        let file = std::io::BufWriter::new(file);
        // serde_json::to_writer(file, &init_log).map_err(|err| DBError::from_serialization(err))?;
        // let v: serde_json::Value = serde_json::from_str(data).map_err(|err| DBError::from_serialization(err))?;
        // let db = KvStore{store:v};
        Ok(KvStore {
            // store: kv_store,
            // path: *path,
            log: Log::new(),
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
