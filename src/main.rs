pub mod lib;

use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, BufRead, Error};

use didyoumean::{edit_distance, insert_and_shift, yank};

const WORDS: &str = include_str!("words.txt");

// Parse command line arguments to get the search term.
#[derive(Parser)]
#[clap(author = "Hisbaan Noorani", version = "1.1.1", about = "Did You Mean: A cli spelling corrector", long_about = None)]
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
    // Correctly output ANSI escape codes on Windows.
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

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
    let search_chars = search_term.chars().collect::<Vec<_>>();
    for word in dictionary {
        // Get edit distance.
        let dist = edit_distance(&search_chars, word);

        // Add to the list if appropriate.
        if dist < top_n_dists[args.number - 1] {
            for i in 0..args.number {
                if dist < top_n_dists[i] {
                    insert_and_shift(&mut top_n_dists, i, dist);
                    insert_and_shift(&mut top_n_words, i, word);
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
