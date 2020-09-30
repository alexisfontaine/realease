use std::fmt::{Display, Formatter, Error as FmtError};
use std::error::Error as StdError;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
	DatabaseOpenFailed,
	DatabaseUpgradeFailed,
	DataInvalid,
	IdbUnavailable,
	LocalStorageCorrupted,
	LocalStorageFull,
	LocalStorageUnavailable,
	QueryFailed,
	ResponseUnexpected,
	StoreInsertFailed,
	StoreRetrieveFailed,
	TokenInvalid,
}


impl StdError for Error {}

impl Display for Error {
	fn fmt (&self, formatter: &mut Formatter<'_>) -> Result<(), FmtError> {
		match self {
			Self::DatabaseOpenFailed => write!(formatter, "The database failed to open."),
			Self::DatabaseUpgradeFailed => write!(formatter, "The database failed to upgrade."),
			Self::DataInvalid => write!(formatter, "Unable to handle some data."),
			Self::IdbUnavailable => write!(formatter, "The indexedDB is not available."),
			Self::LocalStorageCorrupted => write!(formatter, "The state retrieved from the localStorage is invalid."),
			Self::LocalStorageFull => write!(formatter, "Unable to save the state to the localStorage, as it is full."),
			Self::LocalStorageUnavailable => write!(formatter, "The localStorage is not available."),
			Self::QueryFailed => write!(formatter, "The query has failed."),
			Self::ResponseUnexpected => write!(formatter, "The response to the query is unexpected."),
			Self::StoreInsertFailed => write!(formatter, "Unable to insert a new record into the store."),
			Self::StoreRetrieveFailed => write!(formatter, "Unable to retrieve records from the store."),
			Self::TokenInvalid => write!(formatter, "The given token is invalid."),
		}
	}
}
