use std::env::temp_dir;

use fjall::{Database, Keyspace,  PersistMode};

#[derive(Clone)]
pub struct DB {
    db: Database,
    keyspace: Keyspace,
}

#[derive(Debug)]
pub enum Error {
    FjallError(fjall::Error)
}

impl DB {
    pub fn new(path: &str, keyspace: &str) -> Result<DB, Error> {
        let db = Database::builder(path).open().map_err(Error::FjallError)?;
        let tree = db.keyspace(keyspace, fjall::KeyspaceCreateOptions::default).map_err(Error::FjallError)?;
        
        Ok(DB { db: db, keyspace: tree })
    }
}
