use std::process::exit;
use std::net::SocketAddr;
use clap::{Parser, Subcommand};
use kvs::engine::{self, KvStore, KvsEngine, SledKvsEngine};
use kvs::errors::DBError;
use kvs::server::KvsServer;
use kvs::utils;
use log::{info, debug, error};
use sled;
// use tokio::net::TcpListener;
// use tokio::prelude::*;
static DEFAULT_IP: &str="127.0.0.1:4000";
static DEFAULT_ENGINE: &str="sled";

// , default_value_t=DEFAULT_IP.parse().unwrap()


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value_t=DEFAULT_ENGINE.to_string())]
    engine: String,
    #[arg(long, short)]
    addr: SocketAddr,
}

enum Engine {
    KvStore,
    SledKvsEngine,
}

fn run_with_engine<E: KvsEngine>(engine: E, addr: SocketAddr) -> Result<(), DBError> {
    let server = KvsServer::new(engine);
    server.run(addr)
}
// #[tokio::main]
fn main(){
    let args = Args::parse();
    let path = &std::env::current_dir().unwrap();
    // let engine = KvStore::open(&std::env::current_dir().unwrap()).unwrap();
    // Bind the listener to the specified address and port
    if let Ok(config) = utils::read_log(path, &"config".to_string()){
        let mut engine = config["engine_name"].to_string();
        if utils::trim_string(&mut engine).to_string() == args.engine {
            dbg!("kvs-server {}", env!("CARGO_PKG_VERSION"));
            dbg!("Storage engine: {}", args.engine.clone());
            dbg!("Listening on {}", args.addr.clone());
            // println!("with config {}", args.engine);
            let engine_type = match args.engine.as_str() {
                "kvs" => Ok(Engine::KvStore),
                "sled" => Ok(Engine::SledKvsEngine),
                _ => Err(DBError::Server)
            };
            let _ = match engine_type.unwrap() {
                Engine::KvStore => run_with_engine(KvStore::open(path).unwrap(), args.addr),
                Engine::SledKvsEngine => run_with_engine(SledKvsEngine::open(path).unwrap(), args.addr),
            };
        // Ok(())
        }
        else {
            // Err(DBError::Server)
            error!("Wrong engine!");
            exit(1)
        }
    } else {
        dbg!("kvs-server {}", env!("CARGO_PKG_VERSION"));
        dbg!("Storage engine: {}", args.engine.clone());
        dbg!("Listening on {}", args.addr.clone());
        // println!("no config {}", args.engine);
        let engine_type = match args.engine.as_str() {
            "kvs" => Ok(Engine::KvStore),
            "sled" => Ok(Engine::SledKvsEngine),
            _ => Err(DBError::Server)
        };
        let _ = match engine_type.unwrap() {
            Engine::KvStore => run_with_engine(KvStore::open(path).unwrap(), args.addr),
            Engine::SledKvsEngine => run_with_engine(SledKvsEngine::open(path).unwrap(), args.addr),
        };       
        // engine.run(args.addr);
        // Ok(())
    };

    
    

    // let _ = match args.addr {
    //     _ => server.run(),
    // };
}
