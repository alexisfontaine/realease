mod queries;
mod state;
mod store;
mod stores;


pub use self::store::{Migration, Store};
pub use self::stores::{Language, Repository, Release, User};

use js_sys::Array;
use std::collections::HashSet;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Event, IdbDatabase, IdbFactory, IdbRequest, IdbVersionChangeEvent};
use yew::agent::{Dispatched, Dispatcher};
use yew::Callback;
use yew::worker::*;

use super::{Error, Instant};
use super::request::{retrieve_last_releases, Response};
use self::state::State;
use self::stores::{Key, Synchronization};


pub type EventHandler = Closure<dyn FnMut(Event)>;


#[derive(Debug)]
pub enum Input {
	Releases(usize),
	Setup(String),
}

#[derive(Debug)]
pub enum Message {
	FetchData,
	InitializeBackend(Result<IdbDatabase, Error>),
	Releases(HandlerId, Result<Vec<(Release, Repository)>, Error>),
	RestoreState(Result<Option<Synchronization>, Error>),
	SaveResponse((Result<Response, Error>, Synchronization)),
	UpdateDone(Result<usize, Error>),
}

#[derive(Clone, Debug)]
pub enum Output {
	Error(Error),
	LastSynchronization(Instant),
	Ready(bool),
	Releases(Result<Vec<(Release, Repository)>, Error>),
	Synchronizing,
	Updates(usize),
}


pub struct Database {
	database: Option<IdbDatabase>,
	is_synchronizing: bool,
	link: AgentLink<Self>,
	subscribers: HashSet<HandlerId>,
	synchronization: Option<Synchronization>,

	// Keeps the agent alive.
	_dispatcher: Dispatcher<Self>,
}


impl Database {
	pub fn broadcast (&self, output: Output) {
		for subscriber in &self.subscribers
			{ self.link.respond(*subscriber, output.clone()); }
	}
}


impl Agent for Database {
	type Input = Input;

	type Message = Message;

	type Output = Output;

	type Reach = Context<Self>;


	fn create (link: AgentLink<Self>) -> Self {
		if let Err(error) = load(link.callback(Self::Message::InitializeBackend))
			{ link.send_message(Self::Message::InitializeBackend(Err(error))); }

		Self {
			database: None,
			is_synchronizing: false,
			link,
			subscribers: HashSet::new(),
			synchronization: None,
			_dispatcher: Self::dispatcher(),
		}
	}

	fn connected (&mut self, identifier: HandlerId) {
		self.subscribers.insert(identifier);

		if self.is_synchronizing
			{ self.link.respond(identifier, Self::Output::Synchronizing); }
		else if let Some(synchronization) = &self.synchronization
			{ self.link.respond(identifier, Self::Output::LastSynchronization(synchronization.synchronized_at)); }
	}

	fn disconnected (&mut self, identifier: HandlerId) {
		self.subscribers.remove(&identifier);
	}

	fn handle_input (&mut self, input: Self::Input, identifier: HandlerId) {
		match input {
			Self::Input::Releases(limit) => self.send_last_releases(identifier, limit),
			Self::Input::Setup(token) => {
				self.synchronization = Some(Synchronization { token, cursor: None, synchronized_at: Instant::from(Instant::now() - Synchronization::DELAY) });
				self.link.send_message(Self::Message::FetchData);
			}
		}
	}

	fn update (&mut self, message: Self::Message) {
		let output = match message {
			Self::Message::RestoreState(Ok(synchronization)) => {
				Self::Output::Ready(if let Some(synchronization) = synchronization {
					let timeout = (synchronization.synchronized_at.milliseconds() + Synchronization::DELAY).saturating_sub(Instant::now());
					let callback = self.link.callback(|_| Self::Message::FetchData);
					let callback = Closure::<dyn FnMut()>::new(move || callback.emit(()));

					self.synchronization = Some(synchronization);
					window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&callback.into_js_value().into(), timeout as _).unwrap();
					true
				} else {
					self.synchronization = None;
					false
				})
			}
			Self::Message::RestoreState(Err(error)) => {
				self.database = None;
				Self::Output::Error(error)
			}

			Self::Message::InitializeBackend(Ok(database)) => {
				self.database = Some(database);
				self.restore_state();
				return
			}
			Self::Message::InitializeBackend(Err(error)) => {
				Self::Output::Error(error)
			}

			Self::Message::FetchData => {
				let callback = self.link.callback(Self::Message::SaveResponse);
				let synchronization = self.synchronization.take().unwrap();

				spawn_local(async move {
					let result = retrieve_last_releases(synchronization.cursor.as_deref(), &synchronization.token).await;

					callback.emit((result, synchronization));
				});

				if self.is_synchronizing
					{ return }

				self.is_synchronizing = true;
				Self::Output::Synchronizing
			}

			Self::Message::SaveResponse((Ok(response), synchronization)) => {
				self.save_response(synchronization, response);
				return
			}
			Self::Message::SaveResponse((Err(error), synchronization)) => {
				self.synchronization = Some(synchronization);
				Output::Error(error)
			}

			Self::Message::UpdateDone(Ok(updates)) => Output::Updates(updates),
			Self::Message::UpdateDone(Err(error)) => Output::Error(error),
			Self::Message::Releases(identifier, result) => {
				self.link.respond(identifier, Self::Output::Releases(result));
				return
			}
		};

		self.broadcast(output);
	}
}


fn load (callback: Callback<Result<IdbDatabase, Error>>) -> Result<(), Error> {
	let factory = factory()?;
	let mut state = State::initialize()?;

	state.load::<Key>();
	state.load::<Release>();
	state.load::<Repository>();

	// Condition: the version is less than or equal to zero.
	let request = factory.open_with_u32(NAME, state.version()).unwrap();

	let onerror = EventHandler::new({
		let callback = callback.clone();

		move |_| { callback.emit(Err(Error::DatabaseOpenFailed)); }
	});

	let onsuccess = EventHandler::new({
		let callback = callback.clone();

		// Condition: errors should have all been handled.
		move |event: Event| {
			let request = IdbRequest::from(JsValue::from(event.target().unwrap()));

			callback.emit(Ok(IdbDatabase::unchecked_from_js(request.result().unwrap())));
		}
	});

	let onupgradeneeded = Closure::<dyn FnMut(IdbVersionChangeEvent)>::new({
		move |event: IdbVersionChangeEvent| {
			let request = IdbRequest::from(JsValue::from(event.target().unwrap()));
			// Condition: the request is consumed within a transaction.
			let transaction = request.transaction().unwrap();

			if let Err(error) = state.upgrade(&transaction.db(), &transaction)
				{ callback.emit(Err(error)); }
		}
	});

	request.set_onerror(Some(&onerror.into_js_value().into()));
	request.set_onsuccess(Some(&onsuccess.into_js_value().into()));
	request.set_onupgradeneeded(Some(&onupgradeneeded.into_js_value().into()));
	Ok(())
}


pub fn array<T> (values: T) -> JsValue where T: IntoIterator, T::Item: AsRef<JsValue> {
	values.into_iter().collect::<Array>().into()
}

pub fn entry<F, V> (field: F, value: V) -> JsValue where F: Into<JsValue>, V: Into<JsValue> {
	array(&[field.into(), value.into()])
}

fn factory () -> Result<IdbFactory, Error> {
	// Condition: the method is called within a browser context.
	window().unwrap().indexed_db().ok().flatten().ok_or(Error::IdbUnavailable)
}


const NAME: &str = "realease";
