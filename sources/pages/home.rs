use ocean::{Action, Stack};
use std::rc::Rc;
use yew::prelude::*;

use crate::{Error, Instant};
use crate::components::Release;
use crate::database::{Database, Input, Output};


#[derive(Debug)]
pub enum Message {
	Database(Output),
	Refresh
}


pub struct Home {
	database: Box<dyn Bridge<Database>>,
	error: Option<Error>,
	handle_refresh: Callback<MouseEvent>,
	releases: Vec<Rc<Release>>,
	synchronized_at: Option<Instant>,
	updates: usize
}


impl Component for Home {
	type Message = Message;

	type Properties = ();


	fn create (_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut database = Database::bridge(link.callback(Self::Message::Database));

		database.send(Input::Releases(20));

		Self {
			database,
			error: None,
			handle_refresh: link.callback(|_| Self::Message::Refresh),
			releases: Vec::new(),
			synchronized_at: None,
			updates: 0
		}
	}

	fn change (&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update (&mut self, message: Self::Message) -> ShouldRender {
		match message {
			Self::Message::Database(Output::Releases(result)) => match result {
				Ok(releases) => {
					self.releases = releases.into_iter().map(|(release, repository)| Rc::new(Release { repository, release })).collect();
					self.updates = 0;
				}
				Err(error) => self.error = Some(error)
			}
			Self::Message::Database(Output::LastSynchronization(synchronized_at)) => self.synchronized_at = Some(synchronized_at),
			Self::Message::Database(Output::Synchronizing) => self.synchronized_at = None,
			Self::Message::Database(Output::Updates(updates)) => self.updates += updates,
			Self::Message::Database(_) => return false,
			Self::Message::Refresh => {
				self.database.send(Input::Releases(20));

				return false
			}
		}

		true
	}

	fn view (&self) -> Html {
		if let Some(error) = self.error {
			return html! { <Stack tag="main">{"Error: "}{error}</Stack> }
		}

		let status = if let Some(instant) = &self.synchronized_at {
			if self.updates > 0
				{ html!(<Action kind=Action::AnchorInline class="release__updates" onclick=&self.handle_refresh>{self.updates}{" new releases available"}</Action>) } else
				{ html!(<time class="release__status" datetime=instant.iso8601()>{"Synchronized "}<b>{instant.relative_sentence()}</b></time>) }
		} else {
			html!(<span class="release__status">{"Synchronization in progress..."}</span>)
		};

		html! {
			<Stack class="feed" tag="main">
				<article class="releases">
					{status}

					{for self.releases.iter().map(AsRef::as_ref).map(Release::view)}
				</article>
			</Stack>
		}
	}
}
