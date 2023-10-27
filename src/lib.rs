use std::{path::PathBuf, process};

use anyhow::Result;
use git2::Repository;

mod providers;
use providers::github::GitHub;
use providers::Provider;

pub struct GitOpen<'a> {
    /// The Git repository with all its data.
    repository: Repository,
    /// The remote we are interested in. Usually `origin`.
    remote_name: &'a str,

    provider: Box<dyn Provider>,
}

pub enum Entity {
    Repository,
    Branch,
    Commit,
    PullRequest,
}

impl<'a> GitOpen<'a> {
    pub fn new(path: &PathBuf, remote_name: &'a str) -> Self {
        let mut cwd = path.clone();

        let repository = loop {
            match Repository::open(&cwd) {
                Ok(r) => break r,
                Err(_) => {
                    if !cwd.pop() {
                        panic!("Unable to open repository at path or parent: {:?}", path);
                    }
                }
            }
        };

        // TODO(jsgv): Support additional providers.
        let provider = GitHub {};

        Self {
            repository,
            remote_name,
            provider: Box::new(provider),
        }
    }

    pub fn url(&self, entity: Entity) -> Result<String> {
        let remote = self
            .repository
            .find_remote(self.remote_name)
            .unwrap_or_else(|e| {
                println!("Could not retrieve remote: {}", e);
                process::exit(1);
            });

        let remote_url = remote.url().unwrap_or_else(|| {
            println!("Could not retrieve remote url.");
            process::exit(1);
        });

        let head = self.repository.head().unwrap_or_else(|e| {
            println!("Could not retrieve repository head: {}", e);
            process::exit(1);
        });

        let branch = head.shorthand().unwrap_or_else(|| {
            println!("Could not retrieve branch name.");
            process::exit(1);
        });

        let commit = head.target().unwrap_or_else(|| {
            println!("Could not retrieve commit.");
            process::exit(1);
        });

        match entity {
            Entity::Repository => self.provider.web_url(remote_url),
            Entity::Branch => self.provider.branch_url(remote_url, branch),
            Entity::PullRequest => self.provider.pull_request_url(remote_url, branch),
            Entity::Commit => self.provider.commit_url(remote_url, &commit.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GitOpen;
    use std::{env, path::PathBuf};

    #[test]
    fn can_be_created() {
        let p = env::current_dir().unwrap();
        GitOpen::new(&p, "origin");
    }

    #[test]
    fn can_be_created_in_child_path() {
        let mut p = env::current_dir().unwrap();
        p.push("src");
        GitOpen::new(&p, "origin");
    }

    #[test]
    #[should_panic]
    fn panics_correctly() {
        let p = PathBuf::from("/tmp");
        GitOpen::new(&p, "origin");
    }
}
