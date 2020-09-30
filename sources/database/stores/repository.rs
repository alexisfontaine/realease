use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{IdbDatabase, IdbObjectStoreParameters, IdbTransaction};

use crate::database::{entry, Migration, Store};
use crate::helpers::Instant;
use super::User;


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Language {
	pub color: Option<String>,
	pub name: String
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Repository {
	pub created_at: Instant,
	pub description: String,
	pub homepage: String,
	pub identifier: String,
	pub is_archived: bool,
	pub is_disabled: bool,
	pub is_locked: bool,
	pub language: Option<Language>,
	pub name: String,
	pub owner: User,
	pub path: String,
	pub pushed_at: Option<Instant>,
	pub released_at: Option<Instant>,
	pub stargazers: u32,
	pub starred_at: Instant,
	pub updated_at: Instant
}


impl Repository {
	pub const BY_RELEASE: &'static str = RELEASED_AT_INDEX;
}


impl Store for Repository {
	const MIGRATIONS: &'static [Migration] = &[create_store];

	const NAME: &'static str = "repository";


	fn indexed_entries (&self) -> Vec<JsValue> {
		vec![
			entry(PRIMARY_INDEX, &self.identifier),
			entry(RELEASED_AT_INDEX, self.released_at.as_ref().map_or(0, Instant::milliseconds) as f64)
		]
	}
}



fn create_store (name: &str, database: &IdbDatabase, _: &IdbTransaction) -> Result<(), JsValue> {
	let store = database.create_object_store_with_optional_parameters(name, IdbObjectStoreParameters::new().key_path(Some(&PRIMARY_INDEX.into())))?;

	store.create_index_with_str(Repository::BY_RELEASE, RELEASED_AT_INDEX)?;
	Ok(())
}


const PRIMARY_INDEX: &str = "identifier";

const RELEASED_AT_INDEX: &str = "released_at";
