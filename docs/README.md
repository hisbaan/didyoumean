# DidYouMean

DidYouMean (or dym) is a command-line spell checker written in rust utilizing [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance).

## Installation

### From binaries

Check out the [Releases page](https://github.com/hisbaan/didyoumean/releases) for prebuilt versions of `dym`.

### Build from source

Run the following command to build `dym` from source and install it in your home directory. Ensure that you have `$CARGO_HOME/bin/` in your path.

```sh
cargo install --locked dym
```

## Developer Installation

The build dependencies for this project are `git`, `rust`, `rustc`, and `cargo`. First, clone this repository, then run

```sh
cargo run -- <args>
```

where `<args>` are the command-line arguments you would pass the DidYouMean binary.
