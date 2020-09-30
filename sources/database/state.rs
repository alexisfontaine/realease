use serde::{Deserialize, Serialize};
use serde_json::{from_str as deserialize, to_string as serialize};
use std::collections::hash_map::{Entry, HashMap};
use web_sys::{window, IdbDatabase, IdbTransaction, Storage};

use crate::Error;
use super::store::{Migration, Store};


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Snapshot {
	database: u32,
	stores: HashMap<String, u32>
}

pub struct State {
	migrations: HashMap<&'static str, &'static [Migration]>,
	state: Snapshot
}


impl State {
	pub fn initialize () -> Result<Self, Error> {
		let state = if let Some(serialized) = storage()?.get(KEY).ok().flatten()
			{ deserialize(&serialized).map_err(|_| Error::LocalStorageCorrupted)? } else
			{ Snapshot::default() };

		Ok(Self { migrations: HashMap::new(), state })
	}

	pub fn load<T> (&mut self) where T: Store {
		let mut migrations = T::MIGRATIONS;

		if let Some(version) = self.state.stores.get(T::NAME) {
			let version = *version as usize;

			if version == migrations.len()
				{ return }

			migrations = &migrations[version..];
		}

		if let Entry::Vacant(entry) = self.migrations.entry(T::NAME)
			{ entry.insert(migrations); }
	}

	pub fn version (&self) -> u32 {
		let version = self.state.database;

		if self.migrations.is_empty()
			{ version } else
			{ version + 1 }
	}

	pub fn upgrade (&mut self, database: &IdbDatabase, transaction: &IdbTransaction) -> Result<(), Error> {
		let mut state = self.state.clone();
		let migrations = &self.migrations;

		if !migrations.is_empty() {
			for (&name, &migrations) in migrations {
				for migration in migrations {
					if let Err(_) = migration(name, database, transaction)
						{ return Err(Error::DatabaseUpgradeFailed) }
				}

				let applied_migrations = migrations.len() as u32;

				if let Some(version) = state.stores.get_mut(name)
					{ *version += applied_migrations; } else
					{ state.stores.insert(name.to_string(), applied_migrations); }
			}

			state.database += 1;
			// Condition: The `State` type does not contain maps with non-string keys.
			storage()?.set(KEY, &serialize(&state).unwrap()).map_err(|_| Error::LocalStorageFull)?;
			self.state = state;
		}

		Ok(())
	}
}


fn storage () -> Result<Storage, Error> {
	// Condition: the method is called within a browser context.
	window().unwrap().local_storage().ok().flatten().ok_or(Error::LocalStorageUnavailable)
}


const KEY: &str = "realease/database";
