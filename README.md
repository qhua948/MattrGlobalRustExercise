# Rust exercise

The goal of this exercise is to create a REST API in rust that takes incoming requests and stores them where they need to go or returns the requested information.

For simplicity, this exercise does not worry about authenticating the requests.

The objects that are handled by this API are three things: Cryptographic Keys, Schemas, and Credentials.

Cryptographic keys that pass through this API are simply byte arrays that represent public keys.

Schemas describe the structure and data containted in a Credential similar to a database table. For example, a basic birth certificate credential contains a birthdate as an unsigned integer, parent first and last names as strings, the baby first and last name as strings, the birth location, and the issuer's unique id.

Credentials are the data mapped from a schema and signed by a cryptographic key.

Your code should be able to handle these three types of data by storing and retrieving them from a persistent layer like SQLite or LMDB.

Some skeleton code has been provided for you to get you started. All objects must be serializable and parsed as JSON. 

Each object should have an identifier field that is a unsigned 64 bit integer.

The expected operations for REST create, read, update, and delete.

Read should support returning all stored objects, limits, offsets, and get by id.

When completed, submit a PR for review.
