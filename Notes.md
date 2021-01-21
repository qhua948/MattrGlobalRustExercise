# Notes

* SQLite does not support u64 natively, hence, all INTEGER field used when talking to database is u32
  
* Binary data is encoded as base64
  
* Signature verification can be done with algos like HMAC or PKI, for example, PGP is a good implementation to use here,
  skipping due to time constraints

* Database resides in `db.sqlite`, initial schema can be found in `./migrations/`, an initialised copy can be found at `db.sqlite`


# Rooms for improvements

* Code duplication in `dao/mod.rs`, this can be further worked on with some abstraction similar
  to `routes/internal/mod.rs`
* More test cases, due to time constraints, only a few are provided.
* Database config can be parameterised
* Better Database column serialisation
* Proper API documentation with OpenAPI

# Documentation

## Rest API Endpoints

### Credentials

=> GET /credentials?<limit>&<offset> (get_credentials)
=> GET /credentials/<id> (get_credential_by_id)
=> POST /credentials (create_credential)
=> DELETE /credentials/<id> (delete_credential)
=> PUT /credentials (update_credential)

### Schemas

=> GET /schemas?<limit>&<offset> (get_schemas)
=> GET /schemas/<id> (get_schema_by_id)
=> POST /schemas (create_schema)
=> DELETE /schemas/<id> (delete_schema)
=> PUT /schemas (update_schema)

### CryptographicKeys

=> GET /cryptographic_keys?<limit>&<offset> (get_cryptographic_keys)
=> GET /cryptographic_keys/<id> (get_cryptographic_key_by_id)
=> POST /cryptographic_keys (create_cryptographic_key)
=> DELETE /cryptographic_keys/<id> (delete_cryptographic_key)
=> PUT /cryptographic_keys (update_cryptographic_key)

# Schema spec

Schema can be specified via a JSON object with special 'tagged' strings as type specification, supports arbitrary
nesting.

The abstract semantic of the schema is:

```text
{
  "schema": {
    "<key>": "Bool" | "UInt" | "Float" | "String" | "Null" | "List": [self..] | "Map": { <key>: self, ...}, ..
  }
}
```

For example

```json
{
  "schema": {
    "a": "Bool",
    "b": {
      "Map": {
        "c": "Bool"
      }
    },
    "d": {
      "List": [
        {
          "Map": {
            "e": "Float"
          }
        }
      ]
    }
  }
}
```

```json
{
  "a": true,
  "b": {
    "c": false
  },
  "d": [
    {
      "e": 1.1
    }
  ]
}
```

Schema conformance checking is lax on objects, i.e schema will ignore json `objects` with extra fields, but it will
reject json `arrays` with extra item(s).

Example:

Given the following schema:

```json
{
  "schema": {
    "a": "Bool",
    "b": {
      "Map": {
        "c": "Bool"
      }
    },
    "d": {
      "List": [
        {
          "Map": {
            "e": "Float"
          }
        }
      ]
    }
  }
}
```

Then the following is an acceptable data payload:

```json
{
  "a": true,
  "b": {
    "c": false,
    "x": "any"
  },
  "d": [
    {
      "e": "Float"
    }
  ]
}
```

But these following are not:

```json
{
  "a": true,
  "b": {
    "c": false,
    "x": "any"
  },
  "d": [
    {
      "e": "Float"
    },
    "Something"
  ]
}
```

```json
{
  "a": true,
  "b": {
    "c": false,
    "x": "any"
  },
  "d": [
    {},
    "Something"
  ]
}
```

# Request lifecycle example

Start by creating a key and a schema:

```http request
POST localhost:8000/crypotograpic_keys
Content-Type: application/json

{"public_key": "something"}
```

returns: 201

```json
{
  "public_key": "something",
  "id": 1
}
```

Then create a schema:

```http request
POST localhost:8000/schema
Content-Type: application/json

{
    "schema": {
        "a": "Bool"
    }
}
```

returns: 201

```json
{
    "schema": {
        "a": "Bool"
    },
    "id": 12
}
```

With the both ids we can create a credential:
```http request
POST localhost:8000/credential
Content-Type: application/json

{
    "schema_id": 7,
    "public_key_id": 5,
    "finger_print": "something",
    "data": {
        "a": true
    }
}
```

returns: 201

```json
{
    "id": 2,
    "schema_id": 7,
    "public_key_id": 5,
    "finger_print": "something",
    "data": {
        "a": true
    }
}
```

# Postman

I have included a Postman collection to document the APIs, this is by no means a replacement for actual API tests.

All requests samples can be found here.

