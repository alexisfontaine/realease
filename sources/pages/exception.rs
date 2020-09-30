use ocean::{Stack, Title};
use ocean::utils::ne_assign;
use yew::prelude::*;

use crate::Error;


#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
	Error(Error),
	NotFound
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Exception {
	pub kind: Kind
}


#[allow(non_upper_case_globals, non_snake_case)]
impl Exception {
	pub const NotFound: Kind = Kind::NotFound;


	pub const fn Error (error: Error) -> Kind {
		Kind::Error(error)
	}
}


impl Component for Exception {
	type Message = ();

	type Properties = Self;


	fn create (properties: Self::Properties, _: ComponentLink<Self>) -> Self {
		properties
	}

	fn change (&mut self, properties: Self::Properties) -> ShouldRender {
		ne_assign(self, properties)
	}

	fn update (&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		html! {
			<Stack tag="main">
				<article>
					{self.kind.render()}
				</article>
			</Stack>
		}
	}
}

impl Renderable for Kind {
	fn render (&self) -> Html {
		match self {
			Self::Error(error) => html! {
				<>
					<Title>{"Error"}</Title>
					<p>{error.to_string()}</p>
				</>
			},
			Self::NotFound => html! {
				<Title>{"Not Found"}</Title>
			}
		}
	}
}
