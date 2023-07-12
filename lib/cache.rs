use cached::Cached;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Cache {
    table: String,
    conn: Connection,
    cache: HashMap<String, String>,
}

pub struct CacheBuilder {
    table: Option<String>,
    conn: Option<Connection>,
    cache: Option<HashMap<String, String>>,
}

impl CacheBuilder {
    pub fn new() -> CacheBuilder {
        CacheBuilder {
            table: None,
            conn: None,
            cache: None,
        }
    }

    pub fn with_table(mut self, table: String) -> CacheBuilder {
        self.table = Some(table);
        self
    }

    pub fn with_connection(mut self, conn: Connection) -> CacheBuilder {
        self.conn = Some(conn);
        self
    }

    pub fn with_cache(mut self, cache: HashMap<String, String>) -> CacheBuilder {
        self.cache = Some(cache);
        self
    }

    pub fn build(self) -> Result<Cache> {
        Ok(Cache {
            table: self.table.ok_or("Table must be set")?,
            conn: self.conn.ok_or("Connection must be set")?,
            cache: self.cache.unwrap_or_default(),
        })
    }
}
/*
let cache = CacheBuilder::new()
    .with_table("my_table".to_string())
    .with_connection(Connection::open("cache.db")?)
    .with_cache(HashMap::new())
    .build()?;
*/
