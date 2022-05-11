#![cfg_attr(not(feature = "std"), no_std)]

pub enum AvroError {
	#[error("Invalid avro schema: {0}")]
	InvalidSchema(String),
	#[error("Invalid avro records")]
	InvalidRecords(),
}

/// Function to convert a raw schema into serialized Avro schema.
/// If schema is malformed or invalid, returns an error.
/// # Arguments
/// * `raw_schema` - raw schema to be converted
/// # Returns
/// * `Result<(Schema, Vec<u8>), AvroError>` - structured and serialized Avro schema
/// # Examples
/// ```
/// use common_helpers::avro;
/// use common_helpers::types::*;
/// let raw_schema = r#"{"type": "record", "name": "User", "fields": [{"name": "name", "type": "string"}, {"name": "favorite_number", "type": "int"}]}"#;
/// let schema_result = avro::fingerprint_raw_schema(raw_schema);
/// assert!(schema_result.is_ok());
/// let serialized_schema = schema_result.unwrap().1;
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

pub fn parse_str(raw_schema: &str) {}