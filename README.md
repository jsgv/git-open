# git-open

Open git repositories in a web browser with `git open`.

This project was inspired by [git-open](https://github.com/paulirish/git-open).

### Usage

```shell
# default remote name is `origin`
git open

# can specify a different remote
git open upstream

# open current commit
git open -c
```

### Installation

You can clone the repo and install from source. This requires you to have rust installed.

```shell
$ git clone git@github.com:jsgv/git-open.git

$ cd git-open

$ make install
```

Or you can download one of the binaries from the releases section and place it
somewhere in your path. This way `git open` would work wherever your project may
be located.

### Why?

I wanted to learn Rust.
