use std::env;

use anyhow::Result;
use clap::Parser;

use gitopen::{Entity, GitOpen};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, conflicts_with_all = &["branch"])]
    commit: bool,

    #[clap(short, long, value_parser)]
    branch: bool,

    #[clap(short, long, value_parser, default_value = "origin")]
    remote: String,

    /// Local branch name must match remote branch name for pull request.
    #[clap(long = "pr", value_parser)]
    pull_request: bool,

    /// Only print url.
    #[clap(short, long, value_parser)]
    print: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let entity = if args.branch {
        Entity::Branch
    } else if args.commit {
        Entity::Commit
    } else if args.pull_request {
        Entity::PullRequest
    } else {
        Entity::Repository
    };

    let p = env::current_dir()?;

    let go = GitOpen::new(&p, &args.remote);
    let url = go.url(entity)?;

    if args.print {
        println!("{}", url);
    } else {
        open::that(url).unwrap();
    }

    Ok(())
}
