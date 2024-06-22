use clap::{Parser, Subcommand};
use kvs::KvStore;

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
    Set { key: String, value: String },
    #[command(about = "remove value")]
    Rm { key: String },
}

fn main() {
    // let _temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(&std::env::current_dir().unwrap()).unwrap();
    let args = Args::parse();

    let _ = match args.cmd {
        Commands::Get { key } => store.get(key.to_string()),
        Commands::Set { key, value } => store.set(key.to_string(), value.to_string()),
        Commands::Rm { key } => store.remove(key.to_string()).map_err(|_e| panic!()),
    };
}
