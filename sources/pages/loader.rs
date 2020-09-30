use ocean::Stack;
use yew::prelude::*;


pub struct Loader;


impl Component for Loader {
	type Message = ();

	type Properties = ();


	fn create (_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self
	}

	fn change (&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update (&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view (&self) -> Html {
		html! {
			<Stack tag="main">
				<></>
			</Stack>
		}
	}
}
