#![feature(cell_update, once_cell, option_result_contains, or_patterns)]
#![recursion_limit="512"]


mod application;
mod components;
mod database;
mod error;
mod helpers;
mod pages;
mod request;


pub use self::error::Error;

use wasm_bindgen::prelude::*;
use yew::start_app;

use self::application::Application;
use self::helpers::Instant;


#[wasm_bindgen(start)]
pub async fn start () {
	start_app::<Application>();
}


// When the `wee_alloc` feature is enabled, use `WeeAlloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
mod allocator {
	use wee_alloc::WeeAlloc;


	#[global_allocator]
	static ALLOCATOR: WeeAlloc = WeeAlloc::INIT;
}
