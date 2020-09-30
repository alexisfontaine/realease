use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{IdbDatabase, IdbObjectStoreParameters, IdbTransaction};

use crate::helpers::Instant;

use crate::database::{entry, Migration, Store};
use super::User;


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Release {
	pub author: Option<User>,
	pub created_at: Instant,
	pub description: String,
	pub identifier: String,
	pub is_draft: bool,
	pub is_prerelease: bool,
	pub name: String,
	pub published_at: Option<Instant>,
	pub repository: String,
	pub tag: String,
	pub updated_at: Instant
}


impl Release {
	pub const BY_REPOSITORY_AND_CREATED_AT: &'static str = "repository,created_at";
}


impl Store for Release {
	const NAME: &'static str = "release";

	const MIGRATIONS: &'static [Migration] = &[create_store];


	fn indexed_entries (&self) -> Vec<JsValue> {
		vec![
			entry(PRIMARY_INDEX, &self.identifier),
			entry(REPOSITORY_INDEX, &self.repository),
			entry(CREATED_AT_INDEX, self.created_at.milliseconds() as f64)
		]
	}
}


fn create_store (name: &str, database: &IdbDatabase, _: &IdbTransaction) -> Result<(), JsValue> {
	let store = database.create_object_store_with_optional_parameters(name, IdbObjectStoreParameters::new().key_path(Some(&PRIMARY_INDEX.into())))?;

	store.create_index_with_str_sequence(Release::BY_REPOSITORY_AND_CREATED_AT, &entry(REPOSITORY_INDEX, CREATED_AT_INDEX))?;
	Ok(())
}


const CREATED_AT_INDEX: &str = "created_at";

const PRIMARY_INDEX: &str = "identifier";

const REPOSITORY_INDEX: &str = "repository";
