use anyhow::{anyhow, Result};
use git_url_parse::GitUrl;

pub mod github;

pub trait Provider {
    fn branch_url(&self, remote: &str, branch: &str) -> Result<String>;
    fn commit_url(&self, remote: &str, commit: &str) -> Result<String>;
    fn pull_request_url(&self, remote: &str, head: &str) -> Result<String>;

    fn web_url(&self, remote: &str) -> Result<String> {
        let git_url = self.git_url(remote)?;

        let scheme_prefix = match git_url.scheme_prefix {
            true => format!("{}://", git_url.scheme),
            false => String::from("https://"),
        };

        let host = match &git_url.host {
            Some(host) => host.to_string(),
            None => String::new(),
        };

        Ok(format!("{}{}/{}", scheme_prefix, host, git_url.fullname))
    }

    fn git_url(&self, remote: &str) -> Result<GitUrl> {
        let url = match GitUrl::parse(remote) {
            Ok(url) => url,
            Err(err) => {
                return Err(anyhow!(err.to_string()));
            }
        };

        Ok(url)
    }
}
