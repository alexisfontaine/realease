use ocean::router::Anchor;
use std::lazy::SyncLazy;
use web_sys::{window, ScrollBehavior, ScrollToOptions};
use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;
use yew_router::switch::Permissive;

use super::Error;
use super::pages::*;
use super::database::{Database, Output};


pub type ApplicationAnchor = Anchor<ApplicationRoute>;


#[derive(Clone, Debug, PartialEq, Switch)]
pub enum ApplicationRoute {
	#[to = "/!"]
	Home,

	#[to = "/about!"]
	About,

	#[to = "/not-found!"]
	NotFound(Permissive<String>),

	#[to = "/setup!"]
	Setup
}

#[derive(Debug)]
pub enum Message {
	Database(Output),
	Navigation(Route),
}


pub struct Application {
	error: Option<Error>,
	is_loading: bool,
	is_ready: bool,
	router: RouteAgentBridge,
	route: Option<ApplicationRoute>,

	_database: Box<dyn Bridge<Database>>
}


impl ApplicationRoute {
	pub fn redirect (route: Route) -> Self {
		Self::NotFound(Permissive(Some(route.route)))
	}
}


impl Component for Application {
	type Message = Message;

	type Properties = ();


	fn create (_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Application {
			error: None,
			is_loading: true,
			is_ready: false,
			router: RouteAgentBridge::new(link.callback(Self::Message::Navigation)),
			route: None,
			_database: Database::bridge(link.callback(Self::Message::Database))
		}
	}

	fn change (&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update (&mut self, message: Self::Message) -> ShouldRender {
		yew::services::ConsoleService::info(&format!("message {:?}", message));

		match message {
			Self::Message::Database(Output::Ready(is_ready)) => {
				self.is_loading = false;
				self.is_ready = is_ready;
				self.router.send(RouteRequest::GetCurrentRoute);

				return false
			}
			Self::Message::Database(Output::Error(error)) if self.route.is_none() => {
				self.error = Some(error);
				self.is_loading = false;
				self.router.send(RouteRequest::ReplaceRoute(ApplicationRoute::About.into()));

				return false
			}
			Self::Message::Database(Output::Error(Error::QueryFailed | Error::ResponseUnexpected | Error::TokenInvalid)) => return false,
			Self::Message::Database(Output::Error(error)) => self.error = Some(error),
			Self::Message::Database(Output::Updates(_)) => {
				if let Some(ApplicationRoute::Setup) = self.route {
					self.router.send(RouteRequest::ReplaceRoute(ApplicationRoute::Home.into()));
					self.is_ready = true;
				}

				return false
			}
			Self::Message::Database(_) => return false,
			Self::Message::Navigation(route) => {
				let (route, is_redirected) = match ApplicationRoute::switch(route.clone()) {
					Some(ApplicationRoute::Home) if !self.is_ready => (ApplicationRoute::About, true),
					Some(ApplicationRoute::Setup) if self.is_ready => (ApplicationRoute::Home, true),
					Some(route) => (route, false),
					None => (ApplicationRoute::redirect(route), true)
				};

				if is_redirected
					{ self.router.send(RouteRequest::ReplaceRouteNoBroadcast(route.clone().into())); }

				if self.route.contains(&route)
					{ return false }

				self.route = Some(route);
				SCROLL_OPTIONS.with(|options| window().unwrap().scroll_with_scroll_to_options(options));
			}
		}

		true
	}

	fn view (&self) -> Html {
		match (&self.route, self.error, self.is_loading) {
			(None, ..) | (.., true) => html!(<Loader />),
			(Some(ApplicationRoute::About), ..) => html!(<About />),
			(Some(ApplicationRoute::NotFound(_)), ..) => html!(<Exception kind=Exception::NotFound />),
			(_, Some(error), ..) => html!(<Exception kind=Exception::Error(error) />),
			(Some(ApplicationRoute::Home), ..) => html!(<Home />),
			(Some(ApplicationRoute::Setup), ..) => html!(<Setup />)
		}
	}
}


thread_local! {
	static SCROLL_OPTIONS: SyncLazy<ScrollToOptions> = SyncLazy::new(|| {
		let mut options = ScrollToOptions::new();

		options.top(0.).left(0.).behavior(ScrollBehavior::Smooth);
		options
	});
}
