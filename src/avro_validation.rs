#![cfg_attr(not(feature = "std"), no_std)]

use crate::types::*;
use apache_avro::{ schema::Schema};
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
// use std::{collections::HashMap, io::Cursor, str};
// use sp_runtime::{traits::One, DispatchError};
// use sp_std::{collections::btree_map::BTreeMap, convert::TryInto, prelude::*,vec::Vec};

/// Represents error types returned by the `avro` module.
#[derive(thiserror::Error, Debug)]
pub enum AvroError {
	#[error("Invalid avro schema: {0}")]
	InvalidSchema(String),
	#[error("Invalid avro records")]
	InvalidRecords(),
}
// pub fn parse_str(raw_schema: &str) {}

/// Function to convert a raw schema into serialized Avro schema.
/// If schema is malformed or invalid, returns an error.
/// # Arguments
/// * `raw_schema` - raw schema to be converted
/// # Returns
/// * `Result<(Schema, Vec<u8>), AvroError>` - structured and serialized Avro schema
/// # Examples
/// ```
// / use common_helpers::avro;
// / use common_helpers::types::*;
// / let raw_schema = r#"{"type": "record", "name": "User", "fields": [{"name": "name", "type": "string"}, {"name": "favorite_number", "type": "int"}]}"#;
// / let schema_result = avro::fingerprint_raw_schema(raw_schema);
// / assert!(schema_result.is_ok());
// / let serialized_schema = schema_result.unwrap().1;
pub fn fingerprint_raw_schema(raw_schema: &str) -> Result<(Schema, Vec<u8>), AvroError> {
	// Replace below with our own
    let schema_result = Schema::parse_str(raw_schema)?;
	let schema_canonical_form = schema_result.canonical_form();
	Ok((schema_result, schema_canonical_form.as_bytes().to_vec()))
}

/// Function to convert a list of raw schema into serialized Avro schema.
/// If schema is malformed or invalid, it is set to Null.
/// # Arguments
/// * `raw_schema` - raw schema list to be converted
/// # Returns
/// * `Result<(Vec<Schema>, Vec<Vec<u8>>), AvroError>` - structured and serialized Avro schemas
/// # Examples
/// ```
/// use common_helpers::avro;
/// use common_helpers::types::*;
/// let raw_schema = r#"{"type": "record", "name": "User", "fields": [{"name": "name", "type": "string"}, {"name": "favorite_number", "type": "int"}]}"#;
/// let vec_raw_schema: [&str; 1] = [raw_schema];
/// let schema_result = avro::fingerprint_raw_schema_list(&vec_raw_schema);
/// assert!(schema_result.is_ok());
/// let serialized_schemas = schema_result.unwrap().1;
/// ```
pub fn fingerprint_raw_schema_list(
	raw_schema: &[&str],
) -> Result<(Vec<Schema>, Vec<Vec<u8>>), AvroError> {
	let schemas: (Vec<Schema>, Vec<Vec<u8>>) = raw_schema
		.par_iter()
		.map(|r| -> (Schema, Vec<u8>) {
			let schema = fingerprint_raw_schema(r);
			match schema {
				Ok(schema) => schema,
				Err(_error) => (Schema::Null, r.to_string().as_bytes().to_vec()),
			}
		})
		.collect();

	Ok(schemas)
}

///Function to convert a serialized Avro schema into Avro Schema type.
/// If schema is malformed or invalid, returns an error.
/// # Arguments
/// * `serialized_schema` - serialized Avro schema to be converted
/// # Returns
/// * `Result<Schema, AvroError>` - structured Avro schema
/// # Examples
/// ```
/// use common_helpers::avro;
/// use common_helpers::types::*;
/// let raw_schema = r#"{"type": "record", "name": "User", "fields": [{"name": "name", "type": "string"}, {"name": "favorite_number", "type": "int"}]}"#;
/// let serialized_schema = avro::fingerprint_raw_schema(raw_schema);
/// assert!(serialized_schema.is_ok());
/// let schema = serialized_schema.unwrap().1;
/// let translated_schema = avro::translate_schema(schema);
/// assert!(translated_schema.is_ok());
/// ```
pub fn translate_schema(serialized_schema: Vec<u8>) -> Result<Schema, AvroError> {
	let schema_str = str::from_utf8(&serialized_schema);
	match schema_str {
		Ok(schema_str) => {
			let schema = Schema::parse_str(schema_str)?;
			Ok(schema)
		},
		Err(error) => Err(AvroError::InvalidSchema(error.to_string())),
	}
}

///Function to convert a list of serialized Avro schema into Avro Schema type.
/// If schema is malformed or invalid, it is set to Null.
/// # Arguments
/// * `serialized_schema` - list of serialized Avro schema to be converted
/// # Returns
/// * `Result<Vec<Schema>, AvroError>` - structured Avro schema
/// # Examples
/// ```
/// use common_helpers::avro;
/// use common_helpers::types::*;
/// let raw_schema = r#"{"type": "record", "name": "User", "fields": [{"name": "name", "type": "string"}, {"name": "favorite_number", "type": "int"}]}"#;
/// let serialized_schema = avro::fingerprint_raw_schema(raw_schema);
/// assert!(serialized_schema.is_ok());
/// let schema = serialized_schema.unwrap().1;
/// let vec_schema = vec![schema];
/// let translated_schema = avro::translate_schemas(vec_schema);
/// assert!(translated_schema.is_ok());
/// ```
pub fn translate_schemas(serialized_schema: Vec<Vec<u8>>) -> Result<Vec<Schema>, AvroError> {
	let schemas: Vec<Schema> = serialized_schema
		.par_iter()
		.map(|o| -> Schema {
			let schema = translate_schema(o.to_vec());
			match schema {
				Ok(schema) => schema,
				Err(_error) => Schema::Null,
			}
		})
		.collect();

	Ok(schemas)
}