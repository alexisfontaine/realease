use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{IdbDatabase, IdbObjectStoreParameters, IdbTransaction};

use crate::Instant;
use crate::database::{entry, Migration, Store};
use super::User;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Key {
	Synchronization(Synchronization),
	User(User)
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Synchronization {
	pub cursor: Option<String>,
	pub synchronized_at: Instant,
	pub token: String
}


impl Key {
	pub const SYNCHRONIZATION_KEY: &'static str = "synchronization";
}

impl Synchronization {
	pub const DELAY: u64 = 30 * 60 * 1_000;
}


impl Store for Key {
	const NAME: &'static str = "key";

	const MIGRATIONS: &'static [Migration] = &[create_store];


	fn indexed_entries (&self) -> Vec<JsValue> {
		vec![entry(PRIMARY_INDEX, match self {
			Self::Synchronization(..) => SYNCHRONIZATION_KEY,
			Self::User(..) => "user"
		})]
	}
}


fn create_store (name: &str, database: &IdbDatabase, _: &IdbTransaction) -> Result<(), JsValue> {
	database.create_object_store_with_optional_parameters(name, IdbObjectStoreParameters::new().key_path(Some(&PRIMARY_INDEX.into())))?;
	Ok(())
}


const PRIMARY_INDEX: &str = "identifier";

const SYNCHRONIZATION_KEY: &str = "synchronization";
