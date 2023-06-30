use anyhow::{anyhow, Result};
use regex::Regex;

pub trait Provider {
    fn branch_url(&self, remote: &str, branch: &str) -> Result<String>;
    fn commit_url(&self, remote: &str, commit: &str) -> Result<String>;

    /// Converts a git remote into an url that can be opened in a browser.
    /// There are usually 2 main ways to define a git remote.
    /// - http[s]://
    /// - git@
    /// This will attempt to extract the important parts of the remote and create
    /// a valid web url.
    ///
    /// Valid examples:
    /// - git@github.com:jsgv/git-open.git
    /// - git@github.com:jsgv/git-open
    /// - https://github.example.com/jsgv/git-open.git
    /// - https://github.example.com/jsgv/git-open
    fn repository_url(&self, remote: &str) -> Result<String> {
        let r =
            Regex::new(r"((git|ssh|http(s)?)|(git@([\w\.]+)))(:(//)?)([\w\.@:/\-~]+?)(\.git)?$")
                .unwrap();
        let caps = r.captures(remote).unwrap();

        if let (Some(domain), Some(path)) = (caps.get(5), caps.get(8)) {
            return Ok(format!("https://{}/{}", domain.as_str(), path.as_str()));
        } else if let (Some(protocol), Some(path)) = (caps.get(2), caps.get(8)) {
            return Ok(format!("{}://{}", protocol.as_str(), path.as_str()));
        }

        Err(anyhow!("Did not match any patterns for remote: {}", remote))
    }
}

pub struct GitHub {}

impl Provider for GitHub {
    fn branch_url(&self, remote: &str, branch: &str) -> Result<String> {
        let web = self.repository_url(remote)?;
        Ok(format!("{}/tree/{}", web, branch))
    }

    fn commit_url(&self, remote: &str, commit: &str) -> Result<String> {
        let web = self.repository_url(remote)?;
        Ok(format!("{}/commit/{}", web, commit))
    }
}

#[cfg(test)]
mod tests {
    use super::{GitHub, Provider};
    use std::collections::HashMap;

    #[test]
    fn github_branch_url() {
        let gh = GitHub {};
        let mut urls = HashMap::new();

        urls.insert(
            "https://github.com/jsgv/git-open/tree/master",
            "https://github.com/jsgv/git-open.git",
        );

        urls.insert(
            "https://github.com/jsgv/git-open/tree/master",
            "https://github.com/jsgv/git-open",
        );

        urls.insert(
            "https://github.com/jsgv/git-open/tree/master",
            "git@github.com:jsgv/git-open.git",
        );

        for (expected, remote) in &urls {
            let formatted = gh.branch_url(remote, "master").unwrap();
            assert_eq!(expected, &formatted);
        }
    }

    #[test]
    fn github_commit_url() {
        let gh = GitHub {};
        let mut urls = HashMap::new();

        urls.insert(
            "https://github.com/jsgv/git-open/commit/abcdef123456",
            ("https://github.com/jsgv/git-open.git", "abcdef123456"),
        );

        urls.insert(
            "https://github.com/jsgv/git-open/commit/123456abcdef",
            ("git@github.com:jsgv/git-open.git", "123456abcdef"),
        );

        for (expected, (remote, commit_id)) in &urls {
            let formatted = gh.commit_url(remote, commit_id).unwrap();
            assert_eq!(expected, &formatted);
        }
    }

    #[test]
    fn github_repository_url() {
        let gh = GitHub {};
        let mut urls = HashMap::new();

        urls.insert(
            "git@github.com:jsgv/git-open",
            "https://github.com/jsgv/git-open",
        );

        urls.insert(
            "git@github.com:jsgv/git-open.git",
            "https://github.com/jsgv/git-open",
        );

        urls.insert(
            "https://github.com/jsgv/git-open.git",
            "https://github.com/jsgv/git-open",
        );

        urls.insert(
            "https://github.example.com/jsgv/git-open.git",
            "https://github.example.com/jsgv/git-open",
        );

        for (remote, expected) in &urls {
            let formatted = gh.repository_url(remote).unwrap();
            assert_eq!(expected, &formatted);
        }
    }
}
