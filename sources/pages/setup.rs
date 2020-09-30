use ocean::{Action, Anchor, Field, Stack, Title};
use yew::prelude::*;

use crate::Error;
use crate::database::{Database, Input, Output};


pub enum Message {
	Database(Output),
	Submit,
	Update(String)
}


pub struct Setup {
	database: Box<dyn Bridge<Database>>,
	error: Option<Error>,
	handle_update: Callback<String>,
	handle_submit: Callback<FocusEvent>,
	is_loading: bool,
	token: String
}


impl Component for Setup {
	type Message = Message;

	type Properties = ();


	fn create (_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let handle_submit = link.callback(|event: FocusEvent| {
			event.prevent_default();
			Message::Submit
		});

		Self {
			database: Database::bridge(link.callback(Self::Message::Database)),
			error: None,
			handle_submit,
			handle_update: link.callback(Message::Update),
			is_loading: false,
			token: String::new(),
		}
	}


	fn change (&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update (&mut self, message: Self::Message) -> ShouldRender {
		match message {
			Self::Message::Database(Output::Error(error)) if self.is_loading => {
				self.error = Some(error);
				self.is_loading = false;
			}
			Self::Message::Database(_) => return false,
			Self::Message::Submit => {
				self.database.send(Input::Setup(self.token.clone()));
				self.is_loading = true;
			}
			Self::Message::Update(token) => {
				self.error = None;
				self.token = token;
			}
		}

		true
	}

	fn view (&self) -> Html {
		let has_error = self.error.is_some();

		html! {
			<Stack tag="main" class="stretched setup">
				<form onsubmit=&self.handle_submit>
					<Title>{"Let's get you set up!"}</Title>

					<p>
						{"In order to retrieve the last releases from your starred repositories on "}
						<Anchor kind=Anchor::Inline to="https://github.com" open_in_new_context=true>{"GitHub"}</Anchor>
						{", a personal access token is required."}
					</p>

					<p>
						{"You can follow the steps in your "}
						<Anchor kind=Anchor::Inline to="https://github.com/settings/tokens" open_in_new_context=true>{"GitHub account settings"}</Anchor>
						{" to generate one. No scopes should be required, unless you want to also retrieve releases from private repositories you starred."}
					</p>

					<Field
						autofocus=true
						details=self.error.as_ref().map_or_else(String::default, ToString::to_string)
						disabled=self.is_loading
						error=has_error
						label="Personal access token"
						oninput=&self.handle_update />

					<Action effect=Action::Submit kind=Action::ButtonPrimary disabled={has_error || self.token.is_empty()} loading=self.is_loading>{"Save and complete"}</Action>

					<footer class="setup__footer">
						<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
							<path d="M368,192H352V112a96,96,0,1,0-192,0v80H144a64.07,64.07,0,0,0-64,64V432a64.07,64.07,0,0,0,64,64H368a64.07,64.07,0,0,0,64-64V256A64.07,64.07,0,0,0,368,192Zm-48,0H192V112a64,64,0,1,1,128,0Z" />
						</svg>

						<p>
							{"No data is collected. All information are requested directly from "}
							<Anchor kind=Anchor::Inline to="https://docs.github.com/en/graphql" open_in_new_context=true>{"GitHub GraphQL API"}</Anchor>
							{" and stored in your browser."}
						</p>
					</footer>
				</form>
			</Stack>
		}
	}
}
