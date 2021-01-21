use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone)]
pub struct IdObj {
    pub id: Option<u32>
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Credential {
    pub id: Option<u32>,
    pub schema_id: Option<u32>,
    pub public_key_id: Option<u32>,
    pub finger_print: Option<String>,
    pub data: Option<Value>,
}

type SchemaBaseType = HashMap<String, SchemaValueType>;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Schema {
    pub schema: Option<SchemaBaseType>,
    pub id: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum SchemaValueType {
    Bool,
    Int,
    Float,
    String,
    Null,
    List(Vec<SchemaValueType>),
    Map(HashMap<String, SchemaValueType>),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CryptographicKeys {
    pub public_key: Option<String>,
    pub id: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ErrorMessage {
    pub error: &'static str,
}

/// Checks whether the credentials conforms to our schema
pub fn conforms(cred: &Credential, sbt: &SchemaBaseType) -> bool {
    return match cred.data {
        Some(Value::Object(ref m)) => {
            map_verify(m, sbt)
        }
        _ => { false }
    }
}

fn verify_single(v: &Value, sbt: Option<&SchemaValueType>) -> bool {
    if sbt.is_none() {
        return false;
    }
    let sbt = sbt.unwrap();
    match v {
        Value::Array(ref arr) => {
            return if let SchemaValueType::List(ref ls) = sbt {
                vec_verify(arr, ls)
            } else {
                false
            };
        }
        Value::Object(ref m) => {
            return if let SchemaValueType::Map(ref ls) = sbt {
                map_verify(m, ls)
            } else {
                false
            };
        }
        Value::Bool(_) => {
            if let SchemaValueType::Bool = sbt {} else {
                return false;
            }
        }
        Value::Number(ref n) => {
            return match sbt {
                SchemaValueType::Int => { n.is_i64() }
                SchemaValueType::Float => { n.is_f64() }
                _ => { false }
            }
        }
        Value::String(_) => {
            if let SchemaValueType::String = sbt {} else {
                return false;
            }
        }
        Value::Null => {
            if let SchemaValueType::Null = sbt {} else {
                return false;
            }
        }
    }
    true
}

fn vec_verify(m: &Vec<Value>, sbt: &Vec<SchemaValueType>) -> bool {
    if m.len() != sbt.len() {
        return false;
    }
    for (n, i) in m.iter().enumerate() {
        if !verify_single(i, sbt.get(n)) {
            return false;
        }
    }
    true
}

fn map_verify(m: &Map<String, Value>, sbt: &SchemaBaseType) -> bool {
    for i in m.iter() {
        if !verify_single(i.1, sbt.get(i.0)) {
            return false;
        }
    }
    true
}

pub trait WithID {
    fn new_with_new_id(&self, _: u32) -> Self;
}

impl WithID for Schema {
    fn new_with_new_id(&self, i: u32) -> Self {
        Schema {
            id: Some(i),
            schema: self.schema.clone(),
        }
    }
}

impl WithID for CryptographicKeys {
    fn new_with_new_id(&self, i: u32) -> Self {
        CryptographicKeys {
            id: Some(i),
            public_key: self.public_key.clone(),
        }
    }
}

impl WithID for Credential {
    fn new_with_new_id(&self, i: u32) -> Self {
        Credential {
            id: Some(i),
            finger_print: self.finger_print.clone(),
            data: self.data.clone(),
            public_key_id: self.public_key_id.clone(),
            schema_id: self.schema_id.clone(),
        }
    }
}

pub trait Clean {
    fn new_clean(_: u32) -> Self;
}

impl Clean for Schema {
    fn new_clean(i: u32) -> Self {
        Schema {
            id: Some(i),
            schema: None,
        }
    }
}

impl Clean for CryptographicKeys {
    fn new_clean(i: u32) -> Self {
        CryptographicKeys {
            id: Some(i),
            public_key: None,
        }
    }
}

impl Clean for Credential {
    fn new_clean(i: u32) -> Self {
        Credential {
            id: Some(i),
            finger_print: None,
            data: None,
            public_key_id: None,
            schema_id: None,
        }
    }
}

pub trait ProjectData<'a>: Serialize + Deserialize<'a> + WithID + Clean {}

impl ProjectData<'_> for Credential {}

impl ProjectData<'_> for CryptographicKeys {}

impl ProjectData<'_> for Schema {}

#[test]
fn smoke_test_conform_pass() {
    let schema: SchemaBaseType = serde_json::from_str(
        "{
    \"a\": \"Bool\",
    \"b\": {
      \"Map\": {
        \"c\": \"Bool\"
      }
    },
    \"d\": {
      \"List\": [
        {
          \"Map\": {
            \"e\": \"Float\"
          }
        }
      ]
    }
  }").unwrap();
    let data: Value = serde_json::from_str(
        "{
  \"a\": true,
  \"b\": {
    \"c\": false
  },
  \"d\": [
    {
      \"e\": 1.1
    }
  ]
}").unwrap();
    assert!(conforms( &{ Credential {
        id: None,
        schema_id: None,
        public_key_id: None,
        finger_print: None,
        data: Some(data),
    }
    }, &schema));

}

#[test]
fn smoke_test_conform_fail() {
    let schema: SchemaBaseType = serde_json::from_str(
        "{
    \"a\": \"Bool\",
    \"b\": {
      \"Map\": {
        \"c\": \"Bool\"
      }
    },
    \"d\": {
      \"List\": [
        {
          \"Map\": {
            \"e\": \"Int\"
          }
        }
      ]
    }
  }").unwrap();
    let data: Value = serde_json::from_str(
        "{
  \"a\": true,
  \"b\": {
    \"c\": false
  },
  \"d\": [
    {
      \"e\": 1.1
    }
  ]
}").unwrap();
    assert_eq!(false, conforms( &{ Credential {
        id: None,
        schema_id: None,
        public_key_id: None,
        finger_print: None,
        data: Some(data),
    }
    }, &schema));

}

#[test]
/// Will accept extra args in a json Object
fn smoke_test_conform_extra_pass() {
    let schema: SchemaBaseType = serde_json::from_str(
        "{
    \"a\": \"Bool\",
    \"b\": {
      \"Map\": {
        \"c\": \"Bool\"
      }
    },
    \"d\": {
      \"List\": [
        {
          \"Map\": {
            \"e\": \"Float\",
            \"f\": \"Int\"
          }
        }
      ]
    }
  }").unwrap();
    let data: Value = serde_json::from_str(
        "{
  \"a\": true,
  \"b\": {
    \"c\": false
  },
  \"d\": [
    {
      \"e\": 1.1
    }
  ]
}").unwrap();
    assert_eq!(true, conforms( &{ Credential {
        id: None,
        schema_id: None,
        public_key_id: None,
        finger_print: None,
        data: Some(data),
    }
    }, &schema));

}


#[test]
/// Will accept extra args in a json Object
fn smoke_test_conform_vec_extra_fail() {
    let schema: SchemaBaseType = serde_json::from_str(
        "{
    \"a\": \"Bool\",
    \"b\": {
      \"Map\": {
        \"c\": \"Bool\"
      }
    },
    \"d\": {
      \"List\": [
        {
          \"Map\": {
            \"e\": \"Float\"
          }
        },
        \"Bool\"
      ]
    }
  }").unwrap();
    let data: Value = serde_json::from_str(
        "{
  \"a\": true,
  \"b\": {
    \"c\": false
  },
  \"d\": [
    {
      \"e\": 1.1
    }
  ]
}").unwrap();
    assert_eq!(false, conforms( &{ Credential {
        id: None,
        schema_id: None,
        public_key_id: None,
        finger_print: None,
        data: Some(data),
    }
    }, &schema));

}

