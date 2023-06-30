use std::{path::PathBuf, process};

use anyhow::Result;
use git2::Repository;

mod provider;
use provider::{GitHub, Provider};

pub struct GitOpen<'a> {
    repository: Repository,
    remote_name: &'a str,
}

pub enum Entity {
    Repository,
    Branch,
    Commit,
}

impl<'a> GitOpen<'a> {
    pub fn new(path: &PathBuf, remote_name: &'a str) -> Self {
        let mut cwd = path.clone();

        let repository = loop {
            match Repository::open(&cwd) {
                Ok(r) => break r,
                Err(_e) => {
                    if !cwd.pop() {
                        panic!("Unable to open repository at path or parent: {:?}", path);
                    }
                }
            }
        };

        Self {
            repository,
            remote_name,
        }
    }

    pub fn url(&self, entity: Entity) -> Result<String> {
        let provider = GitHub {};

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

        match entity {
            Entity::Repository => provider.repository_url(remote_url),
            Entity::Branch => {
                let branch = head.shorthand().unwrap_or_else(|| {
                    println!("Could not retrieve branch name.");
                    process::exit(1);
                });
                provider.branch_url(remote_url, branch)
            }
            Entity::Commit => {
                let commit = head.target().unwrap_or_else(|| {
                    println!("Could not retrieve commit.");
                    process::exit(1);
                });
                provider.commit_url(remote_url, &commit.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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
