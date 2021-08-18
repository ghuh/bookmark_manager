![CI Status](https://github.com/ghuh/bookmark_manager/actions/workflows/ci/badge.svg?event=push&branch=master)

# Bookmark Manager

or `bm` for short because who doesn't like a good bm...

## Description

The purpose of this project is to create a cross-platform, CLI based, web bookmark manager in Rust that saves bookmarks to a flat file (CSV).

This has several advantages:

- It's not tied to any specific browser, so it is easier to switch back and forth between Firefox and Chrome over the years.
- Flat files are forever. Even if the tools built around them become deprecated so there is no concern of every losing data.
- Flat files play nicely with Git/SCM.

The intention is that the CSV file is stored in Git/SCM, and it can be synced between devices using Git/SCM.

### Details

The "CSV" file is actually `|` separated with 3 columns: URL, DESCRIPTION, and TAGS.  The TAGS column contains a comma separated list.

## Installing

TODO [Create brew formula](https://docs.brew.sh/Adding-Software-to-Homebrew) and include install instructions here

### From source

Requires [installing Rust](https://www.rust-lang.org/tools/install).

```bash
# Installs to ~/.cargo/bin which should be in your path if you installed Rust according to the standard instructions.
cargo install --path .
```

## Usage

Indicate where the application should look for the CSV file either with the `BOOKMARK_MANAGER_CSV` environmental variable.

```bash
# In .bashrc
export BOOKMARK_MANAGER_CSV=<path>
```

or 

```bash
# When running the application
BOOKMARK_MANAGER_CSV=<path> bm ...
```

The CSV will be created if it does not exist at the given path.

### Help

```bash
bm help
```

### Add bookmark

```bash
bm help add
bm add <URL> <DESCRIPTION>
bm a <URL> <DESCRIPTION>

# URLs are validated and must begin with http(s)
bm a https://www.google.com "Google search engine" -t Search --tag Google
bm add https://www.facebook.com "Time sink"

# By default, if BOOKMARK_MANAGER_CSV is in a git repo. A commit will be made after adding a new bookmark. --no-commit to turn off
bm add https://github.com "Source code" --no-commit
```

### Search bookmark

```bash
bm help add
bm search <REGEX>
bm s <REGEX>

# Search with only regex, no tags
# The regex is case insensitive
bm search "search engine"

# Tags are like "and" queries
# Tags are case insensitive
bm s google --tag Search

# Search with only tags, no regex
bm s -t Search
```

On macOS, hold down the command key and double-click on the URL to open it in your default browser.

### Tags

Tags are just a way to organize bookmarks.  Like labels in Gmail.

```bash
bm help tags

# List all tags sorted one per line
# If there are multiple of the same tag with different casing, they will be comma separated on the same line
bm tags
bm t

# Look for a specific tag
bm t |grep "query"
```


## Migrating from browser based bookmark managers

Instructions on how to convert your existing bookmarks.

### Chrome

From a [Chrome HTML export file](https://support.google.com/chrome/answer/96816?hl=en):

```bash
# Folders will be turned into tags
perl -lne 'BEGIN{my @tags=(); print "URL|DESCRIPTION|TAGS"} if (/HREF="([^"]*)"[^>]*>([^<]*)</) {my $url=$1; $url =~ s/\|/%7C/g; my $d=$2; $d =~ s/\|/-/g; print "$url|$d|".join(",", @tags) }; push(@tags, $1) if />([^<]*)<\/H3/; pop(@tags) if /<\/DL>/' 2021_07_22_Chrome.html > bookmarks.csv
```

## Development

Requires [installing Rust](https://www.rust-lang.org/tools/install).

### Test

```bash
# Lint
cargo clippy --all-targets --all-features -- -D warnings
# Runs unit and integration tests
cargo test
```

### Build

#### Development Build

```bash
cargo build
```

#### Release Build

```bash
# To keep a clean build, fail on any compiler warnings, not just errors
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

### Run from source

```bash
cargo run -- add https://www.google.com "Google search engine" -t Search
```

## Reference

- [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
