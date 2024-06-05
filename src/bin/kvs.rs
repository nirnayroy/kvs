use clap::{Parser, Subcommand};

// #![feature(use_extern_macros, macro_helper_hack)]
// #[macro_export(local_inner_macros)]
use kvs::KvStore;
use std::fs;
use std::os;
use std::fs::File;
use std::path::Path;
use tempfile::TempDir;
use std::io::{Read, Write};
use std::io;
use serde_json::{json, Value};
// use serde_json::to_string;
// use assert_cmd::prelude::*;
// use kvs::{KvStore, DBError};
// use predicates::ord::eq;
// use predicates::str::{contains, is_empty, PredicateStrExt};
// use std::process::Command;
// use tempfile::TempDir;
// use walkdir::WalkDir;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "get value")]
    Get { key: String },
    #[command(about = "set value")]
    Set {
        key: String,
        value: String,
        //     is_true: bool
    },
    #[command(about = "remove value")]
    Rm { key: String },
    // Help
}

fn main() {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    // println!("{}", std::env::current_dir().unwrap().display());
    let mut store = KvStore::open(&std::env::current_dir().unwrap()).unwrap();
    let args = Args::parse();

    // // println!("Hello, {}!", args.cmd);
    // let mut store = KvStore::new();
    // // store.set("key".to_string(), "value".to_string());
    // // println!("{:?}", store.get("key".to_string()).unwrap());
    // // // store.remove("key".to_string());
    // // println!("{:?}", store.get("key".to_string()).unwrap());

    // // println!("{:?}", args.cmd);

    match args.cmd {
        Commands::Get { key } => store.get(key.to_string()),
        Commands::Set { key, value } => store.set(key.to_string(), value.to_string()),
        Commands::Rm { key } => store.remove(key.to_string()),
    };

    // let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    // let unique_id = 0348293048;
    // let mut file = File::create(temp_dir.path().join(unique_id.to_string())).expect("Could not create file!");
    // let value = json!({
    //     "previous": unique_id,
    //     "command": "set",
    //     "key": "value",
    //     "value": "value",
    // });
    // file.write_all(value.to_string().as_bytes())
    //     .expect("Cannot write to the file!");
    // println!("file create");

    // let mut data = String::new();
    // let mut f = File::open(temp_dir.path().join(unique_id.to_string())).expect("Unable to open file");
    // f.read_to_string(&mut data).expect("Unable to read string");
    // println!("{}", data);
    // let data_dict: serde_json::Value =
    // serde_json::from_str(&data).expect("JSON was not well-formatted");

    // println!("{:?}", find_highest_numbered_file(temp_dir.path()).unwrap().unwrap());
    // println!("{}", data_dict["key"]);
    // let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    // let mut store = KvStore::open(temp_dir.path()).unwrap();
    // store.set("key1".to_owned(), "value1".to_owned()).unwrap();
    // store.set("key2".to_owned(), "value2".to_owned()).unwrap();
    // drop(store);
    // let value = store.get("key1".to_string()).unwrap().unwrap();
    // println!("{}", value);
    // Command::cargo_bin("kvs")
    //     .unwrap()
    //     .args(&["get", "key1"])
    //     .current_dir(&temp_dir)
    //     .assert()
    //     .success()
    //     .stdout(eq("value1").trim());

    // Command::cargo_bin("kvs")
    //     .unwrap()
    //     .args(&["get", "key2"])
    //     .current_dir(&temp_dir)
    //     .assert()
    //     .success()
    //     .stdout(eq("value2").trim());
}
