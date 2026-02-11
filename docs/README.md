<h1 align="center">DidYouMean</h1>

<p align="center">
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL3-13233a?style=for-the-badge" /></a>
    <a href="https://github.com/hisbaan/didyoumean/actions"><img src="https://img.shields.io/github/workflow/status/hisbaan/didyoumean/tests?style=for-the-badge" /></a>
    <a href="https://aur.archlinux.org/packages/didyoumean/"> <img src="https://img.shields.io/aur/version/didyoumean?color=1793d1&label=AUR&logo=arch-linux&style=for-the-badge" /></a>
    <a href="https://aur.archlinux.org/packages/didyoumean-bin/"> <img src="https://img.shields.io/aur/version/didyoumean-bin?color=1793d1&label=AUR-bin&logo=arch-linux&style=for-the-badge" /></a>
    <a href="https://github.com/NixOS/nixpkgs/tree/master/pkgs/tools/misc/didyoumean"><img src="https://img.shields.io/badge/dynamic/json?color=5277c3&label=NixOS&query=%24.name&url=https%3A%2F%2Fapi.github.com%2Frepos%2Fhisbaan%2Fdidyoumean%2Freleases%2Flatest&style=for-the-badge&logo=NixOS"/></a>
    <a href="https://github.com/hisbaan/homebrew-tap"><img src="https://img.shields.io/badge/dynamic/json?color=fbb040&label=Homebrew&query=%24.name&url=https%3A%2F%2Fapi.github.com%2Frepos%2Fhisbaan%2Fdidyoumean%2Freleases%2Flatest&style=for-the-badge&logo=Homebrew" /></a>
    <a href="https://lib.rs/crates/didyoumean"> <img src="https://img.shields.io/crates/v/didyoumean?color=red&label=crates.io/lib.rs&logo=Rust&style=for-the-badge&logoColor=red" /></a>
</p>

DidYouMean (or `dym`) is a command-line spelling corrector written in rust utilizing a simplified version of [Damerau-Levenshtein distance](https://en.wikipedia.org/wiki/Damerau-Levenshtein_distance). DidYouMean is for those moments when you know what a word sounds like, but you're not quite sure how it's spelled.

<p align="center">
    <img src="img/cyclophosphamide.png" height="250" style="border-radius: 10px; margin: 0.5em;"/>
</p>

## Installation

### Arch Linux (and derivatives)

DidYouMean is available on the AUR as three different packages:

- [didyoumean](https://aur.archlinux.org/packages/didyoumean): Last stable release, built from source (Thank you [orhun](https://github.com/orhun)!).
- [didyoumean-git](https://aur.archlinux.org/packages/didyoumean-git): Last git commit, built from source. This is the most up to date, but the least stable.
- [didyoumean-bin](https://aur.archlinux.org/packages/didyoumean-bin): Last stable release, distributed as a binary. This is only available for `x86_64` at the moment.

You can install it using any AUR helper. Using `paru`, the command would be as follows:

```sh
paru -S <package choice from above>
```

### Homebrew (macOS)

Homebrew is a package manager for macOS. Currently, I have only packaged an x86\_64 binary. The command to install it is as follows:

```sh
brew tap hisbaan/tap
brew install didyoumean
```

### NixOS

[evanjs](https://github.com/evanjs) very kindly packaged `didyoumean` for NixOS. The command to install is as follows:

```sh
nix-env install -iA nixpkgs.didyoumean
```

### Cargo

Run the following command to build `dym` from source and install it in your home directory. Ensure that you have `$CARGO_HOME/bin/` in your path.

```sh
cargo install didyoumean
```

## Developer Installation

The build dependencies for this project are `git`, `rust`, `rustc`, and `cargo`. First, clone this repository, then run

```sh
cargo run -- <args>
```

where `<args>` are the command-line arguments you would pass the DidYouMean binary. Note that this is an unoptimized build contianing debug information so it runs much, much slower.
Adding New Language
===================
Update the language code in the list at 


Update the language wordlist
============================
```
$ cargo run -- --lang ta  --update-langs --wordlist-url https://raw.githubusercontent.com/arcturusannamalai/wordlists/main/
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/dym --lang ta --update-langs --wordlist-url 'https://raw.githubusercontent.com/arcturusannamalai/wordlists/main/'`
Downloading English word list...
Accessing URL: https://raw.githubusercontent.com/arcturusannamalai/wordlists/main//en
[00:00:00] [############################################################################################################################] 4.12MiB/4.12MiB (0s)
Downloading Tamil word list...
Accessing URL: https://raw.githubusercontent.com/arcturusannamalai/wordlists/main//ta
[00:00:00] [############################################################################################################################] 1.73MiB/1.73MiB (0s)
```

Run did you mean:
================
e.g.
```
cargo run -- --lang ta  கலஅ 
   Compiling didyoumean v1.1.4 (/Users/user/devel/rust-in-action/didyoumean)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
     Running `target/debug/dym --lang ta 'கலஅ'`
Did you mean?
1. கலி
2. கலை
3. கல்
4. அ
5. அகல்

```


For more info see the help text and options,
```
$ cargo run -- --help

didyoumean user$ cargo run -- --help
   Compiling didyoumean v1.1.4 (/Users/user/devel/rust-in-action/didyoumean)
    Finished dev [unoptimized + debuginfo] target(s) in 2.24s
     Running `target/debug/dym --help`
didyoumean 1.1.4
Hisbaan Noorani
Did You Mean: A cli spelling corrector

USAGE:
    dym [OPTIONS] [SEARCH_TERM]

ARGS:
    <SEARCH_TERM>
            

OPTIONS:
    -c, --clean-output
            Print a clean version of the output without the title, numbers or colour.

    -h, --help
            Print help information

    -l, --lang <LANG>
            Select the desired language using its locale code. For example, English would have the
            locale code en and French would have the locale code fr. See --print-langs for a list of
            locale codes and the corresponding languages.
            
            [default: en]

    -n, --number <NUMBER>
            Change the number of words the program will print. The default value is five.
            
            [default: 5]

        --print-langs
            Display a list of supported languages and their respective locale codes.

        --update-langs
            Update all language files from the repository specified by CLI @wordlist-url@.

    -v, --verbose
            Print verbose output including the edit distance of the found word to the queried word.

    -V, --version
            Print version information

    -w, --wordlist-url <WORDLIST_URL>
            Wordlist repository URL. The default value is
            'https://raw.githubusercontent.com/hisbaan/wordlists/main'
            
            [default: https://raw.githubusercontent.com/hisbaan/wordlists/main]

    -y, --yank
            Yank (copy) the selected word to the system clipboard. If no word is selected, the
            clipboard will not be altered.

```



