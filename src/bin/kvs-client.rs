use clap::{Parser, Subcommand};
use kvs::engine::{KvsEngine, KvStore};
// use kvs::KvStore;
use kvs::client::KvsClient;
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::exit;

static DEFAULT_IP: &str="127.0.0.1:4000";
// static : SocketAddr = string_ip.parse().unwrap();

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
    
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "get value")]
    Get { 
        key: String, 
        #[arg(long, short, default_value_t=DEFAULT_IP.parse().unwrap())]
        addr: SocketAddr, 
    },
    #[command(about = "set value")]
    Set { 
        key: String, value: String, 
        #[arg(long, short, default_value_t=DEFAULT_IP.parse().unwrap())]
        addr: SocketAddr, 
    },
    #[command(about = "remove value")]
    Rm { 
        key: String, 
        #[arg(long, short, default_value_t=DEFAULT_IP.parse().unwrap())]
        addr: SocketAddr, 
    },
}

fn main() {
    // let _temp_dir = TempDir::new().expect("unable to create temporary working directory");
    // let mut store = KvStore::open(&std::env::current_dir().unwrap()).unwrap();
    let args = Args::parse();
    let _ = match args.cmd {
        Commands::Get { key, addr} => {
            let mut client = KvsClient::connect(addr).unwrap();
            if let Some(value) = client.get(key).unwrap() {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Commands::Set { key, value, addr} => {
            let mut client = KvsClient::connect(addr).unwrap();
            client.set(key, value).unwrap();
        }
        Commands::Rm { key, addr} => {
            let mut client = KvsClient::connect(addr).unwrap();
            if let Err(_) = client.remove(key) {
                eprintln!("Key not found");
                exit(1)
            } else {
                ()
            }
        }
    };
}
