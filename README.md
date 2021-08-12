# Bookmark Manager

or `bm` for short because who doesn't like a good bm...

## Description

The purpose of this project is to create a cross-platform, CLI based, web bookmark manager in Rust that saves bookmarks to a flat file (CSV).

This has several advantages:

- It's not tied to any specific browser so it is easier to switch back and forth between Firefox and Chrome over the years.
- Flat files are forever even if the tools built around them become deprecated so there is no concern of every losing data.
- Flat file play nicely with Git/SCM.

The intention is that the CSV file is stored in Git/SCM and it can be synced between devices using Git/SCM.

### Details

The "CSV" file is actually `|` separated with 3 columns: URL, DESCRIPTION, and TAGS.  The TAGS column contains a comma separated list.

## Installing

TODO from brew

### From source

Requires [installing Rust](https://www.rust-lang.org/tools/install).

```bash
# Installs to ~/.cargo/bin which should be in your path if you installed Rust according to the standard instructions.
cargo install  --features=fail-on-warnings --path .
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

### Help

```bash
bm help
```

### Add bookmark

```bash
bm help add
bm a|add <URL> <DESCRIPTION>

bm a https://www.google.com "Google search engine" -t Search --tag Google
bm add https://www.facebook.com "Time sink"
```

### Search bookmark

```bash
bm help add
bm s|search <REGEX>

bm search "search engine"

# Tags are like "and" queries
bm s google --tag Search
```

On macOS, hold down the command key and double-click on the URL to open it in your default browser.

## Migrating from browser based bookmark managers

Instructions on how to convert your existing bookmarks.

### Chrome

From a [Chrome HTML export file](https://support.google.com/chrome/answer/96816?hl=en):

```bash
perl -lne 'BEGIN{my @tags=(); print "URL|DESCRIPTION|TAGS"} if (/HREF="([^"]*)"[^>]*>([^<]*)</) {my $url=$1; my $d=$2; $d =~ s/\|/-/g; print "$url|$d|".join(",", @tags) }; push(@tags, $1) if />([^<]*)<\/H3/; pop(@tags) if /<\/DL>/' 2021_07_22_Chrome.html > bookmarks.csv
```

## Development

Requires [installing Rust](https://www.rust-lang.org/tools/install).

### Build

#### Development Build

```bash
cargo build
```

#### Release Build

```bash
# To keep a clean build, fail on any compiler warnings, not just errors
cargo build --release --features=fail-on-warnings
```

### Test

```bash
cargo test
```

### Run

```bash
cargo run
```

## Reference

- [Command line apps in Rust](https://rust-cli.github.io/book/index.html)