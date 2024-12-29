use super::KvsEngine;
use serde_json::{self, json};
// use crate::{KvsError, Result};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use sled::{Db, Tree};
use crate::utils;
use crate::errors::DBError;
/// Wrapper of `sled::Db`
#[derive(Clone)]
pub struct SledKvsEngine{
    pub path: Box<Path>,
    // pub db: Db,
    pub tree: Tree,
}

impl SledKvsEngine {
    /// Creates a `SledKvsEngine` from `sled::Db`.
    pub fn open(path: &Path) -> Result<SledKvsEngine, DBError> {
        let config = json!(
            {
                "engine_name": "sled",
            });
        utils::write_log(
            path,
            &"config".to_string(),
            config,
        ).unwrap();
        let db: sled::Db = sled::open(path).unwrap();
        Ok(SledKvsEngine{path : path.into(), tree: db.open_tree("sled_tree").unwrap()})
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<Option<String>, DBError> {
        let tree: &Tree = &self.tree;
        tree.insert(key, value.into_bytes()).map(|_| ()).unwrap();
        tree.flush().unwrap();
        Ok(None)
    }

    fn get(&mut self, key: String) -> Result<Option<String>, DBError> {
        let tree: &Tree = &self.tree;
        Ok(tree
            .get(key).unwrap()
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose().unwrap())
    }

    fn remove(&mut self, key: String) -> Result<Option<String>, DBError> {
        let tree: &Tree = &self.tree;
        if let Ok(_) = tree.remove(key).unwrap().ok_or(DBError::no_key()){
            tree.flush().unwrap();
            Ok(None)
        } else {
            eprintln!("Key not found");
            Err(DBError::no_key())
        }

    }
}