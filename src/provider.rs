use regex::Regex;
use std::error::Error;

pub trait Provider {
    fn branch_url(&self, remote: &str, branch: &str) -> Result<String, Box<dyn Error>>;
    fn commit_url(&self, remote: &str, commit: &str) -> Result<String, Box<dyn Error>>;

    /// Converts a git remote into an url that can be opened in a browser.
    /// There are usually 2 main ways to define a git remote.
    /// - http[s]://
    /// - git@
    /// This will attempt to extract the important parts of the remote and create
    /// a valid web url.
    fn repository_url(&self, remote: &str) -> Result<String, Box<dyn Error>> {
        // git@github.com:jsgv/git-open.git
        let re_git = Regex::new(r"^(\S+)@(\S+):(\S+)(?:\.git$)")?;

        if let (true, Some(captures)) = (re_git.is_match(remote), re_git.captures(remote)) {
            let domain = captures.get(2).unwrap().as_str();
            let repository = captures.get(3).unwrap().as_str();
            return Ok(format!("https://{}/{}", domain, repository));
        }

        // https://github.example.com/jsgv/git-open.git
        // https://github.example.com/jsgv/git-open
        let re_web = Regex::new(r"(http[s]?\S+)(?:\.git$)?")?;

        if let (true, Some(captures)) = (re_web.is_match(remote), re_web.captures(remote)) {
            let web = captures.get(1).unwrap().as_str().to_string();
            return Ok(web);
        }

        Err(format!("Did not match any patterns for remote: {}", remote).into())
    }
}

pub struct GitHub {}

impl Provider for GitHub {
    fn branch_url(&self, remote: &str, branch: &str) -> Result<String, Box<dyn Error>> {
        let web = self.repository_url(remote)?;
        Ok(format!("{}/tree/{}", web, branch))
    }

    fn commit_url(&self, remote: &str, commit: &str) -> Result<String, Box<dyn Error>> {
        let web = self.repository_url(remote)?;
        Ok(format!("{}/commit/{}", web, commit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            "https://github.com/jsgv/git-open",
            "git@github.com:jsgv/git-open.git",
        );

        urls.insert(
            "https://github.com/jsgv/git-open",
            "https://github.com/jsgv/git-open.git",
        );

        urls.insert(
            "https://github.example.com/jsgv/git-open",
            "https://github.example.com/jsgv/git-open.git",
        );

        for (expected, remote) in &urls {
            let formatted = gh.repository_url(remote).unwrap();
            assert_eq!(expected, &formatted);
        }
    }
}
