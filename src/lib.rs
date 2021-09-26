//! Planter is a data generation & seeding framework specially designed to work with MongoDB.
//! You provide the collections you are planning on generating data for, and the entity generators for each document in the collection.
//! The program will handle everything else, you can then import the data into your running database using the `mongorestore` command.
//!
//! BSON Documents is how you'll define the entities that will be generated for each collection.
//! For more information about BSON itself, see [bsonspec.org](http://bsonspec.org) and [BSON Rust](https://docs.rs/bson/2.0.0/bson/).
//!
//! ## Installation
//! ### Requirements
//! - Rust 1.48+
//! - bson = "2.0.0"
//!
//! ### Importing
//! This crate is available on [GitHub](https://github.com/valencian-digital/planter). To use it in your application,
//! simply add it to your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! planter = {git = "https://github.com/valencian-digital/planter", branch = "main"}
//! ```
//!
//!
//!
mod common;
mod execution;
mod seeding;

pub use seeding::seeding::seed_data;
pub use seeding::*;

pub use common::*;
