use std::env;

use anyhow::{anyhow, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{self, HeaderMap, HeaderValue},
};
use serde::Serialize;

use crate::providers::Provider;

pub struct GitHub {}

impl GitHub {
    fn graphql_request(&self) -> Result<RequestBuilder> {
        let gh_token = match env::var("GITHUB_API_TOKEN") {
            Ok(t) => t,
            Err(_err) => return Err(anyhow!("Missing GITHUB_API_TOKEN")),
        };

        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let mut headers = HeaderMap::new();

        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_str(format!("cargo-git-open: {}", VERSION).as_str())?,
        );
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", gh_token).as_str())?,
        );

        Ok(Client::builder()
            .default_headers(headers)
            .build()?
            .post("https://api.github.com/graphql"))
    }
}

impl Provider for GitHub {
    fn branch_url(&self, remote: &str, branch: &str) -> Result<String> {
        let web = self.web_url(remote)?;
        Ok(format!("{}/tree/{}", web, branch))
    }

    fn commit_url(&self, remote: &str, commit: &str) -> Result<String> {
        let web = self.web_url(remote)?;
        Ok(format!("{}/commit/{}", web, commit))
    }

    /// Call GitHub Graphql endpoint and search for a PR for the current head.
    fn pull_request_url(&self, remote: &str, branch: &str) -> Result<String> {
        let git_info = self.git_url(remote)?;

        let variables = pull_requests::Variables {
            owner: git_info.owner.unwrap_or("".into()),
            name: git_info.name,
            head_ref_name: branch.to_string(),
        };

        let payload = PullRequests::build_query(variables);

        let req = self.graphql_request()?;
        let res = req.json(&payload).send()?;

        if res.status() != 200 {
            let body = res.json::<serde_json::Value>()?;
            return Err(anyhow!(body));
        }

        let response_body: Response<pull_requests::ResponseData> = res.json()?;

        // TODO(jsgv): Very ugly, but I don't know how else to do this.
        // A PR for improvement would be greatly appreciated. :)
        //
        let extracting_err = format!("No pull request found for branch '{}'", branch);

        let pr_edges = response_body
            .data
            .ok_or(anyhow!(extracting_err.clone()))?
            .repository
            .ok_or(anyhow!(extracting_err.clone()))?
            .pull_requests
            .edges
            .ok_or(anyhow!(extracting_err.clone()))?;

        let url = &pr_edges
            .first()
            .ok_or(anyhow!(extracting_err.clone()))?
            .as_ref()
            .ok_or(anyhow!(extracting_err.clone()))?
            .node
            .as_ref()
            .ok_or(anyhow!(extracting_err.clone()))?
            .url;

        Ok(url.to_string())
    }
}

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery, Clone, Copy, Debug, Serialize)]
#[graphql(
    schema_path = "src/graphql/github_schema.json",
    query_path = "src/graphql/github_pull_request_query.graphql"
)]
pub struct PullRequests;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn git_web_url() {
        let gh = GitHub {};
        let mut cases = HashMap::new();

        cases.insert(
            "git@github.com:jsgv/git-open",
            "https://github.com/jsgv/git-open",
        );

        cases.insert(
            "git@github.com:jsgv/git-open.git",
            "https://github.com/jsgv/git-open",
        );

        cases.insert(
            "https://github.com/jsgv/git-open.git",
            "https://github.com/jsgv/git-open",
        );

        for (remote, expected) in &cases {
            let url = gh.web_url(remote).unwrap();
            assert_eq!(expected, &url);
        }
    }

    #[test]
    fn github_branch_url() {
        let gh = GitHub {};
        let mut cases = HashMap::new();

        cases.insert(
            "https://github.com/jsgv/git-open/tree/main",
            ("https://github.com/jsgv/git-open.git", "main"),
        );

        cases.insert(
            "https://github.com/jsgv/git-open/tree/master",
            ("https://github.com/jsgv/git-open", "master"),
        );

        cases.insert(
            "https://github.com/jsgv/git-open/tree/main",
            ("git@github.com:jsgv/git-open.git", "main"),
        );

        cases.insert(
            "https://github.com/jsgv/git-open/tree/master",
            ("git@github.com:jsgv/git-open", "master"),
        );

        for (expected, (remote, target_branch)) in &cases {
            let formatted = gh.branch_url(remote, target_branch).unwrap();
            assert_eq!(expected, &formatted);
        }
    }

    #[test]
    fn github_commit_url() {
        let gh = GitHub {};
        let mut cases = HashMap::new();

        cases.insert(
            "https://github.com/jsgv/git-open/commit/abcdef123456",
            ("https://github.com/jsgv/git-open.git", "abcdef123456"),
        );

        cases.insert(
            "https://github.com/jsgv/git-open/commit/123456abcdef",
            ("git@github.com:jsgv/git-open.git", "123456abcdef"),
        );

        for (expected, (remote, commit_id)) in &cases {
            let formatted = gh.commit_url(remote, commit_id).unwrap();
            assert_eq!(expected, &formatted);
        }
    }
}
