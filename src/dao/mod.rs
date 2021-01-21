use std::collections::HashMap;
use std::convert::TryFrom;

use rusqlite::{Connection, Error, NO_PARAMS, Result as SR};
use rusqlite::types::ToSql;
use serde_json::Value;

use crate::datastructures::{Credential, CryptographicKeys, Schema, SchemaValueType};

pub trait ConnectionRestMapping {
    type Target;

    // SQLite only support 64B Signed integer
    fn get_by_id(_: &Connection, id: u32) -> SR<Option<Self::Target>>;
    fn get_all(_: &Connection, limit: Option<u32>, offset: Option<u32>) -> SR<Vec<Self::Target>>;
    fn update(_: &Connection, data: &Self) -> SR<()>;
    fn delete_by_id(_: &Connection, id: u32) -> SR<()>;
    fn create(_: &Connection, data: &Self) -> SR<u32>;
}

impl ConnectionRestMapping for Credential {
    type Target = Self;

    fn get_by_id(conn: &Connection, id: u32) -> Result<Option<Self::Target>, Error> {
        let mut stmt = conn.prepare("SELECT id, schema_id, public_key_id, data, finger_print FROM credentials WHERE id = ?1")?;
        let mut iter = stmt.query_map(&[id], |row| Ok(Credential {
            id: Some(row.get::<_, u32>(0).unwrap()),
            schema_id: Some(row.get::<_, u32>(1).unwrap()),
            public_key_id: Some(row.get::<_, u32>(2).unwrap()),
            data: Some(serde_json::from_str::<Value>(&row.get::<_, String>(3)?).unwrap()),
            finger_print: Some(row.get(4).unwrap()),
        }))?;
        match iter.next() {
            Some(Ok(k)) => Ok(Some(k)),
            _ => SR::Err(Error::QueryReturnedNoRows)
        }
    }

    fn get_all(conn: &Connection, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Self::Target>, Error> {
        let mut raw_stmt = "SELECT id, schema_id, public_key_id, data, finger_print FROM credentials ".to_owned();
        if let Some(i) = limit {
            raw_stmt.push_str(format!(" LIMIT {}", i).as_str())
        }
        if let Some(i) = offset {
            raw_stmt.push_str(format!(" OFFSET {}", i).as_str())
        }
        let mut stmt = conn.prepare(&raw_stmt)?;
        let iter = stmt.query_map(NO_PARAMS, |row| Ok(Credential {
            id: Some(row.get::<_, u32>(0).unwrap() as u32),
            schema_id: Some(row.get::<_, i64>(1).unwrap() as u32),
            public_key_id: Some(row.get::<_, i64>(2).unwrap() as u32),
            data: Some(serde_json::from_str::<Value>(&row.get::<_, String>(3)?).unwrap()),
            finger_print: Some(row.get(4).unwrap()),
        }))?;
        let mut res = vec!();
        for i in iter {
            res.push(i?)
        }
        Ok(res)
    }

    fn update(conn: &Connection, data: &Self) -> Result<(), Error> {
        let mut stmt = conn
            .prepare("UPDATE credentials SET data = ?, finger_print = ? WHERE id = ?")?;
        let res = stmt.execute(
            &[&serde_json::to_string(&data.data).unwrap() as &dyn ToSql,
                &data.finger_print as &dyn ToSql,
                &(&data.id.map(|x| { i64::try_from(x) }).unwrap()).unwrap() as &dyn ToSql
            ])?;
        if res <= 0 {
            return SR::Err(Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn delete_by_id(conn: &Connection, id: u32) -> Result<(), Error> {
        let id = id as i64;
        let mut stmt = conn
            .prepare("DELETE FROM credentials WHERE id = ?1")?;
        let res = stmt.execute(&[&id])?;
        if res <= 0 {
            return SR::Err(Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn create(conn: &Connection, data: &Self) -> Result<u32, Error> {
        let mut stmt = conn
            .prepare("INSERT INTO credentials (data, public_key_id, schema_id, finger_print) VALUES (?, ?, ?, ?)")?;

        stmt.execute(
            &[&serde_json::to_string(&data.data).unwrap() as &dyn ToSql,
                &(data.public_key_id.map(|x| { i64::try_from(x) }).unwrap()).unwrap() as &dyn ToSql,
                &(data.schema_id.map(|x| { i64::try_from(x) }).unwrap()).unwrap() as &dyn ToSql,
                &data.finger_print as &dyn ToSql]
        )?;
        let res = conn.last_insert_rowid() as u32;
        Ok(res)
    }
}

impl ConnectionRestMapping for CryptographicKeys {
    type Target = Self;
    fn get_by_id(conn: &Connection, id: u32) -> SR<Option<Self>> {
        let id = id as i64;
        let mut stmt = conn.prepare("SELECT id, public_key FROM cryptographic_keys WHERE id = ?1")?;
        let mut iter = stmt.query_map(&[id], |row| Ok(CryptographicKeys {
            id: Some(row.get::<_, i64>(0).unwrap() as u32),
            public_key: Some(row.get(1)?),
        }))?;
        match iter.next() {
            Some(Ok(k)) => Ok(Some(k)),
            _ => SR::Err(Error::QueryReturnedNoRows)
        }
    }

    fn get_all(conn: &Connection, limit: Option<u32>, offset: Option<u32>) -> SR<Vec<Self>> {
        let mut raw_stmt = "SELECT id, public_key FROM cryptographic_keys".to_owned();
        if let Some(i) = limit {
            raw_stmt.push_str(format!(" LIMIT {}", i).as_str())
        }
        if let Some(i) = offset {
            raw_stmt.push_str(format!(" OFFSET {}", i).as_str())
        }
        let mut stmt = conn.prepare(&raw_stmt)?;
        let iter = stmt.query_map(NO_PARAMS, |row| Ok(CryptographicKeys {
            id: Some(row.get::<_, i64>(0).unwrap() as u32),
            public_key: Some(row.get(1)?),
        }))?;
        let mut res = vec!();
        for i in iter {
            res.push(i?)
        }
        Ok(res)
    }

    fn update(conn: &Connection, data: &Self) -> SR<()> {
        let mut stmt = conn
            .prepare("UPDATE cryptographic_keys SET public_key = ? WHERE id = ?")?;
        let res = stmt.execute(&[&data.public_key, &data.id.map(|x| x.to_string())])?;
        if res <= 0 {
            return SR::Err(Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn delete_by_id(conn: &Connection, id: u32) -> SR<()> {
        let id = id as i64;
        let mut stmt = conn
            .prepare("DELETE FROM cryptographic_keys WHERE id = ?1")?;
        let res = stmt.execute(&[&id])?;
        if res <= 0 {
            return SR::Err(Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn create(conn: &Connection, data: &Self) -> SR<u32> {
        let mut stmt = conn
            .prepare("INSERT INTO cryptographic_keys (public_key) VALUES (?1)")?;
        stmt.execute(&[&data.public_key])?;
        let res = conn.last_insert_rowid() as u32;
        Ok(res)
    }
}

impl ConnectionRestMapping for Schema {
    type Target = Self;

    fn get_by_id(conn: &Connection, id: u32) -> Result<Option<Self>, Error> {
        let id = id as i64;
        let mut stmt = conn.prepare("SELECT id, schema FROM schemas WHERE id = ?1")?;
        let mut iter = stmt.query_map(&[id], |row| Ok(Self {
            id: Some(row.get::<_, i64>(0).unwrap() as u32),
            schema: Some(serde_json::from_str::<HashMap<String, SchemaValueType>>(&row.get::<_, String>(1)?).unwrap()),
        }))?;
        match iter.next() {
            Some(Ok(k)) => Ok(Some(k)),
            _ => SR::Err(Error::QueryReturnedNoRows)
        }
    }

    fn get_all(conn: &Connection, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Self>, Error> {
        let mut raw_stmt = "SELECT id, schema FROM schemas".to_owned();
        if let Some(i) = limit {
            raw_stmt.push_str(format!(" LIMIT {}", i).as_str())
        }
        if let Some(i) = offset {
            raw_stmt.push_str(format!(" OFFSET {}", i).as_str())
        }
        let mut stmt = conn.prepare(&raw_stmt)?;
        let iter = stmt.query_map(NO_PARAMS, |row| Ok(Self {
            id: Some(row.get::<_, i64>(0).unwrap() as u32),
            schema: Some(serde_json::from_str::<HashMap<String, SchemaValueType>>(&row.get::<_, String>(1)?).unwrap()),
        }))?;
        let mut res = vec!();
        for i in iter {
            res.push(i?)
        }
        Ok(res)
    }

    fn update(conn: &Connection, data: &Self) -> Result<(), Error> {
        let mut stmt = conn
            .prepare("UPDATE schemas SET schema = ? WHERE id = ?")?;
        let res = stmt.execute(&[&serde_json::to_string(&data.schema).unwrap(), &data.id.map(|x| x.to_string()).unwrap()])?;
        if res <= 0 {
            return SR::Err(Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn delete_by_id(conn: &Connection, id: u32) -> Result<(), Error> {
        let id = id as i64;
        let mut stmt = conn
            .prepare("DELETE FROM schemas WHERE id = ?1")?;
        let res = stmt.execute(&[&id])?;
        if res <= 0 {
            return SR::Err(Error::QueryReturnedNoRows);
        }
        Ok(())
    }

    fn create(conn: &Connection, data: &Self) -> Result<u32, Error> {
        let mut stmt = conn
            .prepare("INSERT INTO schemas (schema) VALUES (?1)")?;
        stmt.execute(&[&serde_json::to_string(&data.schema).unwrap()])?;
        let res = conn.last_insert_rowid() as u32;
        Ok(res)
    }
}

