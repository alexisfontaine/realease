use bincode::deserialize;
use ocean::{Anchor, Stack, Title};
use std::lazy::SyncLazy;
use yew::prelude::*;

use crate::application::{ApplicationAnchor, ApplicationRoute};
use crate::components::Release;


pub struct About;


impl Component for About {
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
			<>
				<Stack tag="main" class="about">
					<section class="introduction">
						<article>
							<Title kind=Title::Headline>{"RealEase"}</Title>

							<p class="introduction__subtitle">{"Never miss another release"}</p>

							<ApplicationAnchor kind=ApplicationAnchor::ButtonPrimary route=ApplicationRoute::Setup>{"Get started"}</ApplicationAnchor>
						</article>

						<aside class="releases">
							{for RELEASES.iter().map(Component::view)}
						</aside>
					</section>
				</Stack>

				<Stack class="about__content">
					<article>
						<Title kind=Title::Section>{"How it works"}</Title>

						<p>
							{"RealEase consists in a single page application served as a static web page. It means that, apart from downloading the actual page and its assets, it doesn't receive or send any information from or to its server."}
						</p>

						<p>
							{"The release feed is requested directly from "}
							<Anchor kind=Anchor::Inline to="https://docs.github.com/en/graphql" open_in_new_context=true>{"GitHub GraphQL API"}</Anchor>
							{" using a personal access token. The response data is processed locally and then persisted on "}
							<Anchor kind=Anchor::Inline to="https://developer.mozilla.org/docs/Web/API/IndexedDB_API" open_in_new_context=true>{"IndexedDB"}</Anchor>
							{"."}
						</p>

						<p>
							{"For now, the synchronizations are scheduled to run every half-hour. Depending on the amount of starred repositories that you have, you might reach some limitations of GitHub's API. As a rule of thumb, anything less than a few thousands starred repositories should work just fine!"}
						</p>

						<p>
							{"RealEase is powered by the "}
							<Anchor kind=Anchor::Inline to="https://ocean.alexif.net" open_in_new_context=true>{"Ocean Design System"}</Anchor>
							{". Its source code is available on "}
							<Anchor kind=Anchor::Inline to ="https://github.com/alexisfontaine/realease" open_in_new_context=true>{"GitHub"}</Anchor>
							{" if you want to have a look."}
						</p>
					</article>

					<footer class="about__footer">
						<nav />
					</footer>
				</Stack>
			</>
		}
	}
}


static RELEASES: SyncLazy<Vec<Release>> = SyncLazy::new(|| {
	let mut releases = vec![
		Release {
			release: deserialize(include_bytes!("../../data/bincode.release")).unwrap(),
			repository: deserialize(include_bytes!("../../data/bincode.repository")).unwrap(),
		},
		Release {
			release: deserialize(include_bytes!("../../data/reqwest.release")).unwrap(),
			repository: deserialize(include_bytes!("../../data/reqwest.repository")).unwrap(),
		},
		Release {
			release: deserialize(include_bytes!("../../data/rust.release")).unwrap(),
			repository: deserialize(include_bytes!("../../data/rust.repository")).unwrap(),
		},
		Release {
			release: deserialize(include_bytes!("../../data/serde.release")).unwrap(),
			repository: deserialize(include_bytes!("../../data/serde.repository")).unwrap(),
		},
		Release {
			release: deserialize(include_bytes!("../../data/wasm-bindgen.release")).unwrap(),
			repository: deserialize(include_bytes!("../../data/wasm-bindgen.repository")).unwrap(),
		},
		Release {
			release: deserialize(include_bytes!("../../data/yew.release")).unwrap(),
			repository: deserialize(include_bytes!("../../data/yew.repository")).unwrap(),
		},
	];

	releases.sort_unstable_by(|a, b| b.release.created_at.cmp(&a.release.created_at));
	releases
});
