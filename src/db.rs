use std::env::temp_dir;

use fjall::{Config, PartitionCreateOptions, PersistMode, Slice};

pub fn db() -> Result<(), String> {
    let folder = temp_dir();
    let keyspace = Config::new(folder).open().map_err(|_| "Could not get keyspace")?;

    let partition = keyspace.open_partition("test", PartitionCreateOptions::default()).map_err(|_| "Could not get partition")?;
    partition.insert("hello", "world").map_err(|_| "Could not insert hello")?;
    let hello = partition.get("hello").map_err(|_| "Could not get hello")?;

    println!("hello = {:?}", String::from_utf8(hello.unwrap_or(Slice::new(&[])).to_vec()));

    partition.insert("hellno", "oh no").map_err(|_| "Could not insert hellno")?;
    partition.insert("howdy", "partner").map_err(|_| "Could not insert howdy")?;

    let mut iter = partition.prefix("he");

    while let Some(kv) = iter.next() {
        match kv {
            Ok((k, v)) => println!("k {:?} = v {:?}", String::from_utf8(k.to_vec()), String::from_utf8(v.to_vec())),
            Err(_) => return Err("err with kv".to_string()),
        }
    }

    Ok(())
}
