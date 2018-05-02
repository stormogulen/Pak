use rusqlite::Row;
use std::ops::Deref;
use std::str;


#[derive(Clone, Debug)]
pub struct ResourceInfo {
    path: String,
    size: u32,
    compressed_size: u32,
}

impl ResourceInfo {
    pub fn path(&self) -> u32 {
        self.size
    }

    pub fn compressed_size(&self) -> u32 {
        self.compressed_size
    }

    pub fn from_row(row: &Row) -> ResourceInfo {
        ResourceInfo {
            path: row.get("path"),
            size: row.get("size"),
            compressed_size: row.get("compressed_size"),
        }
    }
}


#[derive(Debug)]
pub struct Resource {
    info: ResourceInfo,
    contents: Vec<u8>,
}

impl Resource {
    pub fn new(info: ResourceInfo, contents: Vec<u8>) -> Resource {
        Resource {
            info: info,
            contents: contents,
        }
    }

    pub fn contents(&self) -> &[u8] {
        &self.contents
    }

    pub fn contents_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.contents)
    }
}


impl Deref for Resource {
    type Target = ResourceInfo;

    fn deref(&self) -> &ResourceInfo {
        &self.info
    }
}
