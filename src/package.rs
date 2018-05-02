use compression;
use Error;
use lz4;
use resource::*;
use rusqlite::{Connection, DatabaseName};
use rusqlite::blob::Blob;
use std::io::{self, Read};
use std::path::Path;


const SCHEMA: &'static str = "
  CREATE TABLE resource (
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    contents BLOB,

    PRIMARY KEY (path)
  );

  PRAGMA application_id = 31113;
  PRAGMA resource.user_version = 1
";


pub struct Package {
    db: Connection,
}

impl Package {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Package, Error> {
        let file_exits = path.as_ref().exists();

        let connection = Connection::open(path)?;

        if !file_exits {
            connection.execute(SCHEMA, &[])?;
        }

        Ok(Package { db: connection })
    }
}
