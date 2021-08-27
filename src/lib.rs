use git2::Branch;
use git2::Repository;
use regex::Regex;
use std::error::Error;

pub struct GitOpen {
    repository: Repository,
}

impl GitOpen {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let repository = Repository::open(".")?;

        Ok(Self { repository })
    }

    pub fn remote_url(
        &self,
        name: &str,
        want_commit: bool,
        want_branch: bool,
    ) -> Result<String, Box<dyn Error>> {
        let remote_info = self.repository.find_remote(name)?;
        let url = remote_info.url().unwrap();
        let mut web_url = convert_git_url(url).unwrap();

        let reference = self.repository.head()?;

        if want_commit {
            let commit_id = reference.peel_to_commit()?.id();

            web_url = self.create_commit_url(web_url, commit_id.to_string())?;
        } else if want_branch {
            let branch = Branch::wrap(reference);
            let branch_name = branch.name().unwrap();

            web_url = self.create_branch_url(web_url, branch_name.unwrap())?;
        }

        Ok(web_url)
    }

    fn create_commit_url(
        &self,
        web_url: String,
        commit_id: String,
    ) -> Result<String, Box<dyn Error>> {
        Ok(format!("{}/commit/{}", web_url, commit_id))
    }

    fn create_branch_url(
        &self,
        web_url: String,
        branch_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        Ok(format!("{}/tree/{}", web_url, branch_name))
    }
}

// https://git-scm.com/docs/git-fetch#_git_urls
fn convert_git_url(git_remote_url: &str) -> Option<String> {
    // git@github.com:jsgv/git-open.git
    let re_url_with_username = Regex::new(r"^(\S+)@(\S+):(\S+)(?:\.git$)").unwrap();

    // https://github.example.com/jsgv/git-open.git
    let re_url_web = Regex::new(r"(http[s]?\S+)(?:\.git$)").unwrap();

    if re_url_with_username.is_match(git_remote_url) {
        for cap in re_url_with_username.captures_iter(git_remote_url) {
            return Some(format!("https://{}/{}", &cap[2], &cap[3]));
        }
    } else if re_url_web.is_match(git_remote_url) {
        for cap in re_url_web.captures_iter(git_remote_url) {
            return Some(format!("{}", &cap[1]));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn converts_git_urls_correctly() {
        let mut urls = HashMap::new();

        // source -> desired

        urls.insert(
            "git@github.com:jsgv/git-open.git",
            String::from("https://github.com/jsgv/git-open"),
        );

        urls.insert(
            "https://github.example.com/jsgv/git-open.git",
            String::from("https://github.example.com/jsgv/git-open"),
        );

        for (key, value) in &urls {
            let formatted = convert_git_url(key).unwrap();
            assert_eq!(value, &formatted);
        }
    }
}
