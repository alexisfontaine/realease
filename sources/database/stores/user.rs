use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
	pub login: String,
	pub path: String,
	pub picture: String
}
