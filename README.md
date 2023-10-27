# git-open

![CI Status](https://github.com/jsgv/git-open/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/cargo-git-open.svg)](https://crates.io/crates/cargo-git-open)

Open git repositories in a web browser with `git open`.

This project was inspired by [git-open](https://github.com/paulirish/git-open).

## Usage

```shell
# open repository in browser
git open

# open current commit
git open -c

# open current branch
git open -b

# open pull request for current branch
# local branch/head must match the pull request branch
# only supported for GitHub
git open --pr

# default remote name is `origin`
# can specify a different remote with `-r`
git open -r upstream

# print only
git open -p
```

## Installation

### Source
You can clone the repo and install from source. This requires you to have rust installed.

```shell
git clone git@github.com:jsgv/git-open.git
cd git-open
make install
```

### Cargo

```shell
cargo install cargo-git-open
```

Or you can download one of the binaries from the releases section and place it
somewhere in your `$PATH`.

## Why?

I wanted to learn Rust.
