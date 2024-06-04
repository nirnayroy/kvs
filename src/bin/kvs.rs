use clap::{Parser, Subcommand};

// #![feature(use_extern_macros, macro_helper_hack)]
// #[macro_export(local_inner_macros)]
use kvs::KvStore;

use std::fs::File;
use tempfile::TempDir;
use std::io::{Read, Write};
use serde_json::{json, Value};
// use serde_json::to_string;

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
    // let args = Args::parse();

    // // println!("Hello, {}!", args.cmd);
    // let mut store = KvStore::new();
    // // store.set("key".to_string(), "value".to_string());
    // // println!("{:?}", store.get("key".to_string()).unwrap());
    // // // store.remove("key".to_string());
    // // println!("{:?}", store.get("key".to_string()).unwrap());

    // // println!("{:?}", args.cmd);

    // match args.cmd {
    //     Commands::Get { key } => store.get(key.to_string()),
    //     Commands::Set { key, value } => store.set(key.to_string(), value.to_string()),
    //     Commands::Rm { key } => store.remove(key.to_string()),
    // };

    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let unique_id = 0348293048;
    let mut file = File::create(temp_dir.path().join(unique_id.to_string())).expect("Could not create file!");
    let value = json!({
        "previous": unique_id,
        "command": "set",
        "key": "value",
        "value": "value",
    });
    file.write_all(value.to_string().as_bytes())
        .expect("Cannot write to the file!");
    println!("file create");

    let mut data = String::new();
    let mut f = File::open(temp_dir.path().join(unique_id.to_string())).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
    let data_dict: serde_json::Value =
    serde_json::from_str(&data).expect("JSON was not well-formatted");

    println!("{}", data_dict["command"]);
    println!("{}", data_dict["key"]);
}
