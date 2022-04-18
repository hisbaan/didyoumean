use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use std::cmp::{min, Ordering};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, BufRead, Error};

#[cfg(unix)]
use nix::unistd::{fork, ForkResult};

const WORDS: &str = include_str!("words.txt");

// Parse command line arguments to get the search term.
#[derive(Parser)]
#[clap(author = "Hisbaan Noorani", version = "1.1.0", about = "Did You Mean: A cli spelling corrector", long_about = None)]
struct Cli {
    search_term: Option<String>,
    #[clap(
        short = 'n',
        long = "number",
        default_value_t = 5,
        help = "Change the number of matches printed"
    )]
    number: usize,
    #[clap(short = 'c', long = "clean-output", help = "Print clean output")]
    clean_output: bool,
    #[clap(short = 'v', long = "verbose", help = "Print verbose output")]
    verbose: bool,
    #[clap(long = "no-color", help = "Print without color")]
    no_color: bool,
    #[clap(
        short = 'y',
        long = "yank",
        help = "Yank (copy) to the system cliboard"
    )]
    yank: bool,
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            1
        }
    });
}

/// Main function to run the application. Return `std::result::Result<(), std::io::Error>`.
fn run_app() -> std::result::Result<(), Error> {
    // Parse args using clap.
    let args = Cli::parse();

    let mut search_term = String::new();

    // Check if nothing was passed in as the search term.
    if args.search_term == None {
        // Check if stdin is empty, produce error if so.
        if atty::is(atty::Stream::Stdin) {
            let mut cmd = clap::Command::new("dym [OPTIONS] <SEARCH_TERM>");
            let error = cmd.error(
                clap::ErrorKind::MissingRequiredArgument,
                format!(
                    "The {} argument was not provided.\n\n\tEither provide it as an argument or pass it in from standard input.",
                    "<SEARCH_TERM>".green()
                )
            );
            clap::Error::exit(&error);
        } else {
            // Read search_term from standard input if stdin is not empty.
            let stdin = io::stdin();
            stdin.lock().read_line(&mut search_term).unwrap();
        }
    } else {
        // Unwrap Option<String> that was read from the client.
        search_term = args.search_term.unwrap();
    }

    // Get dictionary of words from words.txt.
    let dictionary = WORDS.split('\n');

    // Create mutable vecs for storing the top n words.
    let mut top_n_words = vec![""; args.number];
    let mut top_n_dists = vec![search_term.len() * 10; args.number];

    // Loop over the words in the dictionary, run the algorithm, and
    // add to the list if appropriate
    for word in dictionary {
        // Get edit distance.
        let dist = edit_distance(&search_term, word);

        // Add to the list if appropriate.
        if dist < top_n_dists[args.number - 1] {
            for i in 0..args.number {
                if dist < top_n_dists[i] {
                    top_n_dists = insert_and_shift(top_n_dists, i, dist);
                    top_n_words = insert_and_shift(top_n_words, i, word);
                    break;
                }
            }
        }
    }

    // Print out results.
    if !args.clean_output && !args.no_color {
        println!("{}", "Did you mean?".blue().bold());
    } else if args.no_color {
        println!("{}", "Did you mean?".bold());
    }
    let mut items = vec!["".to_string(); args.number];
    for i in 0..args.number {
        let mut output: String = "".to_string();
        let indent = args.number.to_string().len();

        // Add numbers if not clean.
        if !args.clean_output && !args.no_color {
            output.push_str(&format!(
                "{:>indent$}{} ",
                (i + 1).to_string().purple(),
                ".".purple()
            ));
        } else if args.no_color {
            output.push_str(&format!("{:>indent$}. ", (i + 1).to_string()));
        }

        // Add words in order of edit distance.
        output.push_str(&top_n_words[i]);

        // Add edit distance if verbose.
        if args.verbose {
            output.push_str(&format!(" (edit distance: {})", top_n_dists[i]));
        }

        // Print concatenated string.
        items[i] = output;
    }

    // If the yank argument is set, copy the item to the clipboard.
    if args.yank {
        // Print prompt
        println!(
            "{} {}",
            "?".yellow(),
            "[↑↓ to move, ↵ to select, esc/q to cancel]".bold()
        );
        // Get the chosen argument.
        let chosen = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_opt()?;

        // Print out items since dialoguer clears.
        for item in items {
            println!("  {}", item);
        }

        match chosen {
            // If the chosen arguemnt is valid.
            Some(index) => {
                yank(top_n_words[index]);
                println!(
                    "{}",
                    format!("\"{}\" copied to clipboard", top_n_words[index]).green()
                );
            }
            // If no argument is chosen.
            None => {
                println!("{}", "No selection made".red());
                std::process::exit(1);
            }
        }
    } else {
        // If yank is not set, print out all the items.
        for item in items {
            println!("{}", item);
        }
    }

    Ok(())
}

/// Copy `string` to the system clipboard
///
/// # Arguments
///
/// * `string` - the string to be copied.
fn yank(string: &str) {
    let platform = std::env::consts::OS;
    if vec![
        "linux",
        "freebsd",
        "netbsd",
        "dragonfly",
        "netbsd",
        "openbsd",
        "solaris",
    ]
    .contains(&platform)
    {
        // The platform is linux/*bsd and is likely using X11 or Wayland.
        // There is a fix needed for clipboard use in cases like these.
        // The clipboard is cleared on X11/Wayland after the process that set it exist.
        // To combat this, we will fork and keep a process aroudn until the clipboard
        // is cleared.
        // Ideally, this wouldn't be an issue but it was a conscious design decision
        // on X11/Wayland
        #[cfg(unix)]
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(string.to_owned()).unwrap();

                // Keep the process running until the clipboard changes.
                loop {
                    let clipboard = format!("{:?}", ctx.get_contents());
                    std::thread::sleep(std::time::Duration::from_secs(10));
                    if clipboard != string {
                        std::process::exit(0);
                    }
                }
            }
            Err(_) => {
                println!("{}", "Error: Clipboard fork failed".red());
                std::process::exit(1);
            }
            _ => {}
        }
    } else {
        // The platform is NOT running X11/Wayland and thus, we don't have to handle
        // the clipboard clearing behaviour.
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(string.to_owned()).unwrap();
    }
}

/// Return a vec with `element` inserted at `index` and the rest of the vec shifted.
///
/// # Arguments
///
/// * `list` - A vec to be shifted down
/// * `index` - The index at which to insert `element`
/// * `element` - The element to insert at `index`
///
/// # Examples
///
/// ```
/// let to_shift = vec![0, 1, 2, 3, 4];
/// let shifted = insert_and_shift(to_shift, 2, 11);
///
/// assert_eq!(shifted, vec![0, 1, 11, 2, 3]);
/// ```
fn insert_and_shift<T: Copy>(list: Vec<T>, index: usize, element: T) -> Vec<T> {
    if index > list.len() - 1 {
        return list;
    }

    let mut temp = list.clone();

    for i in 0..list.len() {
        match i.cmp(&index) {
            Ordering::Greater => temp[i] = list[i - 1],
            Ordering::Less => temp[i] = list[i],
            Ordering::Equal => temp[i] = element,
        }
    }

    temp
}

/// Return the edit distance between `search_term` and `known_term`.
/// Currently implemented using a modified version of
/// [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance).
///
/// # Arguments
///
/// * `search_term` - The first string to compare
/// * `known_term` - The second string to compare
///
/// # Examples
///
/// ```
/// let dist = edit_distance("sitting", "kitten");
/// assert_eq!(dist, 3)
/// ```
fn edit_distance(search_term: &str, known_term: &str) -> usize {
    // Set local constants for repeated use later.
    let n = search_term.len() + 1;
    let m = known_term.len() + 1;
    let search_chars: Vec<char> = search_term.chars().collect();
    let known_chars: Vec<char> = known_term.chars().collect();

    // Setup matrix 2D vector.
    let mut mat = vec![vec![0; m]; n];

    // Initialize values of the matrix.
    for i in 1..n {
        mat[i][0] = i;
    }
    for i in 1..m {
        mat[0][i] = i;
    }

    // Run the algorithm.
    for i in 1..n {
        for j in 1..m {
            let mut sub_cost = 1;
            if search_chars[i - 1] == known_chars[j - 1] {
                sub_cost = 0;
            }

            mat[i][j] = min(
                mat[i - 1][j - 1] + sub_cost, // substitution cost
                min(
                    mat[i - 1][j] + 1, // deletion cost
                    mat[i][j - 1] + 1, // insertion cost
                ),
            );
            if i > 1
                && j > 1
                && search_chars[i - 1] == known_chars[j - 2]
                && search_chars[i - 2] == known_chars[j - 1]
            {
                mat[i][j] = min(
                    mat[i][j],
                    mat[i - 2][j - 2] + 1, // transposition cost
                );
            }
        }
    }

    // Return the bottom left corner of the matrix.
    mat[n - 1][m - 1]
}
