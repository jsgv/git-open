extern crate clap;
extern crate open;

use clap::{load_yaml, App};
use gitopen::GitOpen;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yaml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    let remote_name = matches.value_of("remote").unwrap();
    let is_flag_print = matches.is_present("print");
    let is_flag_commit = matches.is_present("commit");
    let is_flag_branch = matches.is_present("branch");

    let git_open = GitOpen::new()?;

    let web_url = match git_open.remote_url(remote_name, is_flag_commit, is_flag_branch) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    if is_flag_print {
        println!("{}", web_url);
    } else {
        open::that(web_url).unwrap();
    }

    Ok(())
}
