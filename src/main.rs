use clap::Parser;
use gitopen::{Entity, GitOpen};
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, conflicts_with_all = &["branch"])]
    commit: bool,

    #[clap(short, long, value_parser)]
    branch: bool,

    #[clap(short, long, value_parser, default_value = "origin")]
    remote_name: String,

    #[clap(short, long, value_parser)]
    print: bool,

    #[clap()]
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let entity = if args.branch {
        Entity::Branch
    } else if args.commit {
        Entity::Commit
    } else {
        Entity::Repository
    };

    let path = match &args.path {
        Some(path) => path,
        _ => ".",
    };

    let go = GitOpen::new(path, &args.remote_name);
    let url = go.url(entity)?;

    if args.print {
        println!("{}", url);
    } else {
        open::that(url).unwrap();
    }

    Ok(())
}
