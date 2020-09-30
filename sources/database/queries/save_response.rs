use std::sync::Arc;
use std::cell::Cell;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::{window, Event, IdbObjectStore, IdbRequest, IdbTransactionMode};

use crate::{Error, Instant};
use crate::database::{array, Database, EventHandler, Key, Message, Output, Release, Repository, Store, Synchronization};
use crate::request::Response;


impl Database {
	pub fn save_response (&mut self, mut synchronization: Synchronization, response: Response) {
		let database = self.database.as_ref().unwrap();
		let callback = self.link.callback(Message::UpdateDone);
		let transaction = database.transaction_with_str_sequence_and_mode(&array(&[JsValue::from(Repository::NAME), JsValue::from(Release::NAME), JsValue::from(Key::NAME)]), IdbTransactionMode::Readwrite).unwrap();
		let updates = Arc::new(Cell::new(0));

		let oncomplete = EventHandler::new({
			let updates = updates.clone();
			let callback = callback.clone();

			move |_| { callback.emit(Ok(updates.get())); }
		});

		let onerror = EventHandler::new({
			move |_| { callback.emit(Err(Error::StoreRetrieveFailed)); }
		});

		transaction.set_oncomplete(Some(&oncomplete.into_js_value().into()));
		transaction.set_onerror(Some(&onerror.into_js_value().into()));

		let release_store = transaction.object_store(Release::NAME).unwrap();
		let repository_store = transaction.object_store(Repository::NAME).unwrap();

		for edge in response.repositories {
			repository_store.put(&edge.repository.into_record()).unwrap();

			for release in edge.releases {
				let record = release.into_record();
				let request = release_store.get_key(&release.identifier.into()).unwrap();

				let onsuccess = EventHandler::new({
					let updates = updates.clone();

					move |event: Event| {
						let request = IdbRequest::from(JsValue::from(event.target().unwrap()));
						let store = IdbObjectStore::from(JsValue::from(request.source().unwrap()));
						let result = request.result().unwrap();

						if result.is_undefined() {
							store.add(&record).unwrap();
							updates.update(|count| count + 1);
						} else {
							store.put(&record).unwrap();
						}
					}
				});

				request.set_onsuccess(Some(&onsuccess.into_js_value().into()));
			}
		}

		let pagination = response.pagination;

		if pagination.has_next_page {
			synchronization.cursor = pagination.cursor;
			self.synchronization = Some(synchronization.clone());
			self.link.send_message(Message::FetchData);
		} else {
			let instant = Instant::new();

			synchronization.synchronized_at = instant;
			synchronization.cursor = None;
			self.synchronization = Some(synchronization.clone());
			self.is_synchronizing = false;
			self.broadcast(Output::LastSynchronization(instant));

			let callback = Closure::<dyn FnMut()>::new({
				let callback = self.link.callback(|_| Message::FetchData);

				move || callback.emit(())
			});

			window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&callback.into_js_value().into(), Synchronization::DELAY as _).unwrap();
		};

		transaction
			.object_store(Key::NAME).unwrap()
			.put(&Key::Synchronization(synchronization).into_record()).unwrap();
	}
}
