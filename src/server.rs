use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{BufReader, BufWriter, Write};
use std::os::unix::thread;
use log::{debug, error, info};
use crate::errors::DBError;
use serde_json::de::{Deserializer, IoRead};
use crate::engine::KvsEngine;
use crate::protocol::{GetResponse, Request, SetResponse, RemoveResponse};
use tokio;

pub struct KvsServer<E: KvsEngine>{
    engine: E,
}

impl<E: KvsEngine> KvsServer<E>{   

    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<(), DBError> {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // let handle = thread::spawn();
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client");
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<(), DBError> {
        let peer_addr = tcp.peer_addr().unwrap();
        let reader = BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        macro_rules! send_resp {
            ($resp:expr) => {{
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp).unwrap();
                writer.flush().unwrap();
                debug!("Response sent to {}: {:?}", peer_addr, resp);
            }};
        }

        for req in req_reader {
            let req = req.unwrap();
            debug!("Receive request from {}: {:?}", peer_addr, req);
            let resp = match req {
                Request::Get { key } => send_resp!(match self.engine.get(key) {
                    Ok(value) => GetResponse::Ok(value),
                    Err(e) => GetResponse::Err(format!("error")),
                }),
                Request::Set { key, value } => send_resp!(match self.engine.set(key, value) {
                    Ok(_) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(format!("error")),
                }),
                Request::Remove { key } => send_resp!(match self.engine.remove(key) {
                    Ok(_) => RemoveResponse::Ok(()),
                    Err(e) => RemoveResponse::Err(format!("error")),
                }),
            };
        }
        Ok(())
    }
}