use git2::Repository;
use std::{error::Error, process};

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
    // @todo
    // MergeRequest,
}

impl<'a> GitOpen<'a> {
    pub fn new(path: &str, remote_name: &'a str) -> Self {
        let repository = Repository::open(path).unwrap_or_else(|_| {
            println!("Unable to open repository at path: {:?}", path);
            process::exit(1);
        });

        Self {
            repository,
            remote_name,
        }
    }

    pub fn url(&self, entity: Entity) -> Result<String, Box<dyn Error>> {
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
                    println!("Could not retrieve branch name");
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

    #[test]
    fn can_be_created() {
        GitOpen::new(".", "origin");
    }

    #[test]
    #[should_panic]
    fn panics_correctly() {
        GitOpen::new("/tmp", "origin");
    }
}
