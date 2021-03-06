# map

Runs command for every item in stdin. Similar to xargs.

## Use:

```bash
$ echo hey | map ,, echo ,, ,,
hey hey
```

```bash
$ (echo hey; echo there) | map a echo a a a
hey hey hey
there there there
```

```bash
$ echo hey | map a echo a, a, a | map a echo a, a, a.
hey, hey, hey, hey, hey, hey, hey, hey, hey.
```

```bash
$ ls | map ,, realpath ,,
/Users/a/d/map/Cargo.lock
/Users/a/d/map/Cargo.toml
/Users/a/d/map/readme.md
/Users/a/d/map/src
/Users/a/d/map/target
```

```bash
# using space as a separator
$ echo hey you fhqwhgads | map -s " " ,, echo ,,
hey
you
fhqwhgads
```

## Installation

1. You'll need the [rust compiler](https://www.rust-lang.org/en-US/install.html).
2. Clone this repo, then run `cargo install`.
