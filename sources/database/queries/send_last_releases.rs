use crossbeam_channel::bounded as channel;
use wasm_bindgen::JsValue;
use web_sys::{Event, IdbCursorDirection, IdbCursorWithValue, IdbRequest};
use yew::worker::HandlerId;

use crate::Error;
use crate::database::{entry, Database, EventHandler, Message, Release, Repository, Store};


impl Database {
	pub fn send_last_releases (&self, identifier: HandlerId, limit: usize) {
		let database = self.database.as_ref().unwrap();
		let callback = self.link.callback(move |result| Message::Releases(identifier, result));
		let transaction = database.transaction_with_str_sequence(&entry(Repository::NAME, Release::NAME)).unwrap();
		let (sender, receiver) = channel(limit);

		let oncomplete = EventHandler::new({
			let callback = callback.clone();

			move |_| { callback.emit(Ok(receiver.try_iter().collect())); }
		});

		let onerror = EventHandler::new({
			move |_| { callback.emit(Err(Error::StoreRetrieveFailed)); }
		});

		transaction.set_oncomplete(Some(&oncomplete.into_js_value().into()));
		transaction.set_onerror(Some(&onerror.into_js_value().into()));

		let release_store = transaction.object_store(Release::NAME).unwrap();
		let repository_store = transaction.object_store(Repository::NAME).unwrap();
		let release_index = release_store.index(Release::BY_REPOSITORY_AND_CREATED_AT).unwrap();
		let repository_index = repository_store.index(Repository::BY_RELEASE).unwrap();
		let request = repository_index.open_cursor_with_range_and_direction(&JsValue::UNDEFINED, IdbCursorDirection::Prev).unwrap();

		let onsuccess = EventHandler::new({
			let mut count = 1;

			move |event: Event| {
				let request = IdbRequest::from(JsValue::from(event.target().unwrap()));
				let result = request.result().unwrap();

				if result.is_null()
					{ return }

				let cursor = IdbCursorWithValue::from(result);
				let repository = Repository::from_record(cursor.value().unwrap());

				if let Some(released_at) = &repository.released_at {
					let request = release_index.get(&entry(&repository.identifier, released_at.milliseconds() as f64)).unwrap();
					// Bypasses the limitation of `FnMut` which prevents consuming moved values.
					let mut repository = Some(repository);

					let onsuccess = EventHandler::new({
						let sender = sender.clone();

						move |event: Event| {
							let request = IdbRequest::from(JsValue::from(event.target().unwrap()));
							let release = Release::from_record(request.result().unwrap());
							// Condition: This expression is only evaluated once.
							let repository = repository.take().unwrap();

							// Condition: The channel is opened and never exceeds its capacity.
							sender.try_send((release, repository)).unwrap();
						}
					});

					request.set_onsuccess(Some(&onsuccess.into_js_value().into()));

					if count == limit
						{ return }

					count += 1;
				}

				cursor.continue_().unwrap();
			}
		});

		request.set_onsuccess(Some(&onsuccess.into_js_value().into()));
	}
}
