use crate::errors::DBError;
use crate::protocol::{GetResponse, RemoveResponse, Request, SetResponse};

use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Key value store client
pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, DBError> {
        let tcp_reader = TcpStream::connect(addr).unwrap();
        let tcp_writer = tcp_reader.try_clone().unwrap();
        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
            writer: BufWriter::new(tcp_writer),
        })
    }

    /// Get the value of a given key from the server.
    pub fn get(&mut self, key: String) -> Result<Option<String>, DBError> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key }).unwrap();
        self.writer.flush().unwrap();
        let resp = GetResponse::deserialize(&mut self.reader).unwrap();
        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(msg) => Err(DBError::server()),
        }
    }

    /// Set the value of a string key in the server.
    pub fn set(&mut self, key: String, value: String) -> Result<(), DBError> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value }).unwrap();
        self.writer.flush().unwrap();
        let resp = SetResponse::deserialize(&mut self.reader).unwrap();
        match resp {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(msg) => Err(DBError::server()),
        }
    }

    /// Remove a string key in the server.
    pub fn remove(&mut self, key: String) -> Result<(), DBError> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key }).unwrap();
        self.writer.flush().unwrap();
        let resp = RemoveResponse::deserialize(&mut self.reader).unwrap();
        match resp {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(msg) => Err(DBError::server()),
        }
    }
}
