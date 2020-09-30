use wasm_bindgen::JsValue;
use web_sys::{Event, IdbRequest};

use crate::Error;
use crate::database::{Database, EventHandler, Message, Key, Store};


impl Database {
	pub fn restore_state (&self) {
		let database = self.database.as_ref().unwrap();
		let callback = self.link.callback(Message::RestoreState);
		let transaction = database.transaction_with_str(Key::NAME).unwrap();

		let onerror = EventHandler::new({
			let callback = callback.clone();

			move |_| { callback.emit(Err(Error::StoreRetrieveFailed)); }
		});

		transaction.set_onerror(Some(&onerror.into_js_value().into()));

		let store = transaction.object_store(Key::NAME).unwrap();
		let request = store.get(&Key::SYNCHRONIZATION_KEY.into()).unwrap();

		let onsuccess = EventHandler::new({
			move |event: Event| {
				let request = IdbRequest::from(JsValue::from(event.target().unwrap()));
				let result = request.result().unwrap();

				if result.is_undefined()
					{ callback.emit(Ok(None)); }
				else if let Key::Synchronization(synchronization) = Key::from_record(result)
					{ callback.emit(Ok(Some(synchronization))); }
			}
		});

		request.set_onsuccess(Some(&onsuccess.into_js_value().into()));
	}
}
