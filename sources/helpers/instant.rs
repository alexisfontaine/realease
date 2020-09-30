use chrono::{DateTime, Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::hint::unreachable_unchecked;
use std::str::FromStr;

use crate::Error;


#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Instant(u64);

struct Unit {
	divider: u64,
	format: &'static str,
	has_prefix: bool,
	has_suffix: bool,
	threshold: u64
}


impl Instant {
	pub const fn from (milliseconds: u64) -> Self {
		Self(milliseconds)
	}

	pub fn new () -> Self {
		Self::from(Self::now())
	}

	pub fn iso8601 (&self) -> String {
		let milliseconds = self.0;
		let date = NaiveDateTime::from_timestamp((milliseconds / 1_000) as _, 0)
			.checked_add_signed(Duration::milliseconds((milliseconds % 1_000) as _)).unwrap();

		format!("{}Z", date)
	}

	pub const fn milliseconds (&self) -> u64 {
		self.0
	}

	pub fn relative (&self) -> String {
		self.relative_parts().1
	}

	pub fn relative_sentence (&self) -> String {
		let (difference, mut sentence, unit) = self.relative_parts();

		if difference.is_positive() {
			if unit.has_suffix
				{ sentence.push_str(" ago"); }
		} else if unit.has_prefix {
			sentence.insert_str(0, "in ");
		}

		sentence
	}


	#[cfg(target_arch = "wasm32")]
	pub fn now () -> u64 {
		js_sys::Date::now() as _
	}
	#[cfg(not(target_arch = "wasm32"))]
	pub fn now () -> u64 {
		std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH).unwrap()
			.as_millis() as _
	}

	fn relative_parts (&self) -> (i64, String, &'static Unit) {
		let difference = Self::now() as i64 - self.0 as i64;
		let amplitude = difference.abs() as u64;

		for unit in UNITS {
			if amplitude <= unit.threshold
				{ return (difference, unit.format.replace("{}", &(amplitude / unit.divider).to_string()), unit) }
		}

		// Safety: The last unit has an impassable threshold.
		unsafe { unreachable_unchecked() }
	}
}

impl Unit {
	const fn from (threshold: u64, divider: u64, format: &'static str, has_prefix: bool, has_suffix: bool) -> Self {
		Unit {
			divider: divider.saturating_mul(1000),
			format,
			has_prefix,
			has_suffix,
			threshold: threshold.saturating_mul(1000),
		}
	}

	const fn new (threshold: u64, divider: u64, format: &'static str) -> Self {
		Unit::from(threshold, divider, format, true, true)
	}
}


impl FromStr for Instant {
	type Err = Error;


	fn from_str (encoded: &str) -> Result<Self, Self::Err> {
		Ok(Self::from(DateTime::parse_from_rfc3339(encoded).map_err(|_| Error::DataInvalid)?.timestamp_millis() as _))
	}
}


const UNITS: &[Unit] = &[
	Unit::from(45, 1, "right now", false, false),
	Unit::new(120, 60, "a few minutes"),
	Unit::new(3_600, 60, "{} min"),
	Unit::new(7_200, 3_600, "a few hours"),
	Unit::new(86_400, 3_600, "{} hours"),
	Unit::new(172_800, 86_400, "a few days"),
	Unit::new(2_592_000, 86_400, "{} days"),
	Unit::new(3_888_000, 2_592_000, "about one month"),
	Unit::new(31_536_000, 2_592_000, "{} months"),
	Unit::new(47_304_000, 31_536_000, "about one year"),
	Unit::new(u64::MAX, 31_536_000, "{} years"),
];


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn checks () {
		assert_eq!(UNITS[UNITS.len()].threshold, u64::MAX);
	}

	#[test]
	fn relative () {
		println!("{}", Instant::from(Instant::new().0).relative());
		println!("{}", Instant::from(Instant::new().0 - 60_000).relative());
		println!("{}", Instant::from(Instant::new().0 - 86_400_000).relative());
	}
}
