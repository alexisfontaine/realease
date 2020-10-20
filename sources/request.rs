use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::TryFrom;
use std::lazy::SyncLazy;

use crate::Error;
use crate::database::{Language, Release, Repository, User};


#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
	#[serde(rename = "endCursor")]
	pub cursor: Option<String>,
	pub has_next_page: bool
}

#[derive(Clone, Debug, Serialize)]
pub struct RepositoryEdge {
	pub releases: Vec<Release>,
	pub repository: Repository,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "raw::Response")]
pub struct Response {
	pub count: u32,
	pub pagination: Pagination,
	pub repositories: Vec<RepositoryEdge>
}


impl Release {
	fn try_from (repository: &str, release: raw::Release) -> Result<Self, Error> {
		let created_at = release.createdAt.parse()?;
		let updated_at = release.updatedAt.parse()?;
		let tag = release.tagName;

		Ok(Self {
			author: release.author.map(Into::into),
			created_at,
			description: release.descriptionHTML.as_deref().map_or_else(String::default, parse_description),
			identifier: release.id,
			is_draft: release.isDraft,
			is_prerelease: release.isPrerelease,
			name: release.name.and_then(discard_empty).unwrap_or_else(|| tag.clone()),
			published_at: release.publishedAt.as_deref().map(str::parse).transpose().ok().flatten(),
			repository: repository.to_string(),
			tag,
			updated_at
		})
	}
}

impl TryFrom<raw::StarredRepositoryEdge> for RepositoryEdge {
	type Error = Error;


	fn try_from (edge: raw::StarredRepositoryEdge) -> Result<Self, Self::Error> {
		let node = edge.node;
		let identifier = node.id;
		let nodes = node.releases.nodes;
		let mut releases = Vec::<Release>::with_capacity(nodes.len());

		for node in nodes
			{ releases.push(Release::try_from(&identifier, node)?); }

		let created_at = node.createdAt.parse()?;
		let starred_at = edge.starredAt.parse()?;
		let updated_at = node.updatedAt.parse()?;
		let released_at = releases.first().map(|release| release.created_at);

		Ok(Self {
			releases,
			repository: Repository {
				created_at,
				description: parse_description(&node.descriptionHTML),
				homepage: node.homepageUrl.unwrap_or_default(),
				identifier,
				is_archived: node.isArchived,
				is_disabled: node.isDisabled,
				is_locked: node.isLocked,
				language: node.primaryLanguage,
				name: node.name,
				owner: node.owner.into(),
				path: node.resourcePath,
				pushed_at: node.pushedAt.as_deref().map(str::parse).transpose().ok().flatten(),
				released_at,
				stargazers: node.stargazers.totalCount,
				starred_at,
				updated_at
			}
		})
	}
}

impl TryFrom<raw::Response> for Response {
	type Error = Error;


	fn try_from (response: raw::Response) -> Result<Self, Error> {
		let connection = response.data.viewer.starredRepositories;
		let edges = connection.edges;
		let mut repositories = Vec::<RepositoryEdge>::with_capacity(edges.len());

		for edge in edges
			{ repositories.push(RepositoryEdge::try_from(edge)?); }

		Ok(Self {
			count: connection.totalCount,
			pagination: connection.pageInfo,
			repositories
		})
	}
}

impl From<raw::User> for User {
	fn from (user: raw::User) -> Self {
		Self {
			login: user.login,
			path: user.resourcePath,
			picture: user.avatarUrl
		}
	}
}


pub async fn retrieve_last_releases (cursor: Option<&str>, token: &str) -> Result<Response, Error> {
	let response = CLIENT
		.post("https://api.github.com/graphql")
		.bearer_auth(token)
		.json(&json!({
			"query": "query($c:String){viewer{starredRepositories(after:$c first:100ownedByViewer:false){edges{node{owner{avatarUrl(size:256)login resourcePath}primaryLanguage{color name}releases(last:1){nodes{author{avatarUrl(size:256)login resourcePath}createdAt descriptionHTML id isDraft isPrerelease name publishedAt tagName updatedAt}}stargazers{totalCount}createdAt descriptionHTML homepageUrl id isArchived isDisabled isLocked name pushedAt resourcePath updatedAt}starredAt}pageInfo{endCursor hasNextPage}totalCount}}}",
			"variables": {
				"c": cursor
			}
		}))
		.send()
		.await
		.map_err(|_| Error::QueryFailed)?;

	match response.status().as_u16() {
		200 => Ok(response.json().await.unwrap()),
		401 => Err(Error::TokenInvalid),
		_ => Err(Error::ResponseUnexpected)
	}
}


static CLIENT: SyncLazy<Client> = SyncLazy::new(Default::default);


fn discard_empty (value: String) -> Option<String> {
	if value.is_empty()
		{ None } else
		{ Some(value) }
}

fn parse_description (description: &str) -> String {
	// TODO: handle emojis and anchors.
	description.trim().trim_start_matches("<div>").trim_end_matches("</div>").to_string()
}


#[allow(non_snake_case)]
mod raw {
	use super::*;


	#[derive(Deserialize)]
	pub struct Data {
		pub viewer: Viewer
	}

	#[derive(Deserialize)]
	pub struct Release {
		pub author: Option<User>,
		pub createdAt: String,
		pub descriptionHTML: Option<String>,
		pub id: String,
		pub isDraft: bool,
		pub isPrerelease: bool,
		pub name: Option<String>,
		pub publishedAt: Option<String>,
		pub tagName: String,
		pub updatedAt: String
	}

	#[derive(Deserialize)]
	pub struct ReleaseConnection {
		pub nodes: Vec<Release>
	}

	#[derive(Deserialize)]
	pub struct Repository {
		pub createdAt: String,
		pub descriptionHTML: String,
		pub homepageUrl: Option<String>,
		pub id: String,
		pub isArchived: bool,
		pub isDisabled: bool,
		pub isLocked: bool,
		pub name: String,
		pub owner: User,
		pub primaryLanguage: Option<Language>,
		pub pushedAt: Option<String>,
		pub releases: ReleaseConnection,
		pub resourcePath: String,
		pub stargazers: StargazerConnection,
		pub updatedAt: String
	}

	#[derive(Deserialize)]
	pub struct Response {
		pub data: Data
	}

	#[derive(Deserialize)]
	pub struct StargazerConnection {
		pub totalCount: u32
	}

	#[derive(Deserialize)]
	pub struct StarredRepositoryConnection {
		pub edges: Vec<StarredRepositoryEdge>,
		pub pageInfo: Pagination,
		pub totalCount: u32
	}

	#[derive(Deserialize)]
	pub struct StarredRepositoryEdge {
		pub node: Repository,
		pub starredAt: String
	}

	#[derive(Deserialize)]
	pub struct User {
		pub avatarUrl: String,
		pub login: String,
		pub resourcePath: String
	}

	#[derive(Deserialize)]
	pub struct Viewer {
		pub starredRepositories: StarredRepositoryConnection
	}
}
