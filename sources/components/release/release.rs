use ocean::{Anchor, Title};
use ocean::utils::ne_assign;
use web_sys::window;
use yew::prelude::*;

use crate::database;


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Release {
	pub release: database::Release,
	pub repository: database::Repository,
}


impl Component for Release {
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
		let Self { release, repository } = self;
		let document = window().unwrap().document().unwrap();
		let content = document.create_element("div").unwrap();
		let description = document.create_element("p").unwrap();
		let owner = &repository.owner;
		let owner_url = "https://github.com".to_string() + &owner.path;
		let repository_url = "https://github.com".to_string() + &repository.path;
		let picture_alternative = "Avatar of ".to_string() + &owner.login;
		let url = format!("https://github.com{}/releases/tag/{}", &repository.path, &release.tag);

		content.set_inner_html(&release.description);
		description.set_class_name("repository__description");
		description.set_inner_html(&repository.description);

		html! {
			<>
				<time class="release__date" datetime=release.created_at.iso8601()>
					<b>{release.created_at.relative()}</b>

					{" ago"}
				</time>

				<article class="release">
					<header class="repository">
						<h4 class="repository__title">
							<img class="repository__avatar" src=&owner.picture alt=picture_alternative loading="lazy" />

							<Anchor kind=Anchor::Inline to=owner_url open_in_new_context=true>{&owner.login}</Anchor>
							{" / "}
							<Anchor kind=Anchor::Inline to=repository_url open_in_new_context=true>{&repository.name}</Anchor>
						</h4>

						<section class="repository__details">
							<div>
								{for repository.language.as_ref().map(|language| {
									let style = "background-color:".to_string() + language.color.as_ref().map_or("#eee", String::as_str);

									html! {
										<>
											<span class="repository__language" style=style />
											{" "}
											{&language.name}
										</>
									}
								})}
							</div>

							<div>
								<svg class="icon icon--star" viewBox="0 0 16 16">
									<path d="M8.001.92c.28 0 .549.166.673.418l1.882 3.815 4.21.612c.277.04.519.244.605.51a.763.763 0 01-.189.769l-3.046 2.97.719 4.192a.763.763 0 01-.298.733.763.763 0 01-.79.057l-3.766-1.98-3.766 1.98a.763.763 0 01-.79-.057.763.763 0 01-.298-.732l.72-4.195L.819 7.044a.763.763 0 01-.19-.77.763.763 0 01.606-.51l4.21-.611 1.883-3.815A.763.763 0 018.001.92z" />
								</svg>
								{" "}
								{repository.stargazers}
							</div>

							<span>
								{"Updated "}
								<time datetime=repository.updated_at.iso8601()>{repository.updated_at.relative_sentence()}</time>
							</span>
						</section>
					</header>

					{Html::VRef(description.into())}

					<article class="release__content">
						<div class="release__content__container">
							<Title kind=Title::SubSection>
								<Anchor to=url open_in_new_context=true>
									<svg class="icon icon--release" viewBox="0 0 16 16">
									<path fill-rule="evenodd" d="M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z" />
									</svg>

									<span>{&release.name}</span>
								</Anchor>
							</Title>

							{Html::VRef(content.into())}
						</div>
					</article>
				</article>
			</>
		}
	}
}
