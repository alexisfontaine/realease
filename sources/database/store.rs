use bincode::{deserialize, serialize};
use js_sys::{Object, Reflect, Uint8Array};
use serde::{Deserialize, Serialize};
use std::iter::once;
use wasm_bindgen::JsValue;
use web_sys::{IdbDatabase, IdbTransaction};

use super::{array, entry};


pub type Migration = fn (&str, &IdbDatabase, &IdbTransaction) -> Result<(), JsValue>;


pub trait Store: for<'de >Deserialize<'de> + Serialize {
	const NAME: &'static str;

	const MIGRATIONS: &'static [Migration];


	fn from_record (record: JsValue) -> Self {
		let serialized = Reflect::get(&record, &CONTENT_INDEX.into()).unwrap();

		deserialize(&Uint8Array::to_vec(&serialized.into())).unwrap()
	}

	fn into_record (&self) -> JsValue {
		let serialized = entry(CONTENT_INDEX, Uint8Array::from(serialize(self).unwrap().as_slice()));

		Object::from_entries(&array(once(serialized).chain(self.indexed_entries()))).unwrap().into()
	}

	fn indexed_entries (&self) -> Vec<JsValue>;
}


const CONTENT_INDEX: &str = "content";
