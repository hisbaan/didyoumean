pub mod lib;

use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use phf::phf_map;
use reqwest;
use std::{
    cmp::min,
    fs,
    io::{self, BufRead, Error, Write},
};
use tokio;

use didyoumean::{edit_distance, insert_and_shift, yank};

static LOCALES: phf::Map<&'static str, &'static str> = phf_map! {
    "af" => "Afrikaans",
    "sq" => "Albanian",
    "ar" => "Arabic",
    "eu" => "Basque",
    "be" => "Belarusian",
    "bg" => "Bulgarian",
    "ca" => "Catalan",
    "zh" => "Chinese",
    "hr" => "Croatian",
    "cs" => "Czech",
    "da" => "Danish",
    "nl" => "Dutch",
    "en" => "English",
    "et" => "Estonian",
    "fo" => "Faeroese",
    "fa" => "Farsi",
    "fi" => "Finnish",
    "fr" => "French",
    "gd" => "Gaelic",
    "de" => "German",
    "el" => "Greek",
    "he" => "Hebrew",
    "hi" => "Hindi",
    "hu" => "Hungarian",
    "is" => "Icelandic",
    "id" => "Indonesian",
    "ga" => "Irish",
    "it" => "Italian",
    "ja" => "Japanese",
    "ko" => "Korean",
    "lv" => "Latvian",
    "lt" => "Lithuanian",
    "mk" => "Macedonian",
    "ml" => "Malalyalam",
    "ms" => "Malaysian",
    "mt" => "Maltese",
    "no" => "Norwegian",
    "pl" => "Polish",
    "pt" => "Portugese",
    "pa" => "Punjabi",
    "rm" => "Rhaeto-Romanic",
    "ro" => "Romanian",
    "ru" => "Russian",
    "sr" => "Serbian",
    "sk" => "Slovak",
    "sl" => "Slovenian",
    "sb" => "Sorbian",
    "es" => "Spanish",
    "sv" => "Swedish",
    "th" => "Thai",
    "ts" => "Tsonga",
    "tn" => "Tswana",
    "tr" => "Turkish",
    "ua" => "Ukranian",
    "ur" => "Urdu",
    "ve" => "Venda",
    "vi" => "Vietnamese",
    "cy" => "Welsh",
    "xh" => "Xhosa",
    "ji" => "Yiddish",
    "zu" => "Zulu",
};

static SUPPORTED_LANGS: phf::Map<&'static str, &'static str> = phf_map! {
    "en" => "English",
};

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
    #[clap(
        short = 'y',
        long = "yank",
        help = "Yank (copy) to the system cliboard"
    )]
    yank: bool,
    #[clap(
        short = 'l',
        long = "lang",
        help = "Select the desired language using the locale code (en, fr, sp, etc.)",
        default_value = "en"
    )]
    lang: String,
    #[clap(
        long = "print-supported-langs",
        help = "Display a list of supported languages"
    )]
    print_langs: bool,
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

    if args.print_langs {
        println!("Supported Languages:");
        for key in SUPPORTED_LANGS.keys() {
            println!(" - {}: {}", key, SUPPORTED_LANGS.get(key).clone().unwrap());
        }
        std::process::exit(0);
    }

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

    match args.lang.as_str() {
        "en" => {
            // English word list
            fetch_word_list(
                args.lang.to_owned(),
                format!(
                    "https://raw.githubusercontent.com/hisbaan/wordlists/main/{}",
                    args.lang.to_owned(),
                ),
            );
        }
        _ => {
            // Not supported
            // Initialize new command.
            let mut cmd = clap::Command::new("dym [OPTIONS] <SEARCH_TERM>");

            // Whether or not locale code is valid.
            let error_string = if LOCALES.contains_key(args.lang.as_str()) {
                format!(
                    "There is currently no word list for {}",
                    LOCALES.get(args.lang.as_str()).cloned().unwrap()
                )
            } else {
                format!("{} is not a recognized localed code", args.lang)
            };

            // Set error.
            let error = cmd.error(clap::ErrorKind::MissingRequiredArgument, error_string);

            // Exit with error.
            clap::Error::exit(&error);
        }
    }

    // Get word list. The program will only get here if/when this is a valid word list.
    let word_list =
        fs::read_to_string(dirs::data_dir().unwrap().join("didyoumean").join(args.lang))
            .expect("Error reading file");

    // Get dictionary of words from words.txt.
    let dictionary = word_list.split('\n');

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
    if !args.clean_output {
        println!("{}", "Did you mean?".blue().bold());
    }
    let mut items = vec!["".to_string(); args.number];
    for i in 0..args.number {
        let mut output: String = "".to_string();
        let indent = args.number.to_string().len();

        // Add numbers if not clean.
        if !args.clean_output {
            output.push_str(&format!(
                "{:>indent$}{} ",
                (i + 1).to_string().purple(),
                ".".purple()
            ));
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

/// Fetch the word list specified by `lang` from https://github.com/hisbaan/wordlists
#[tokio::main]
async fn fetch_word_list(lang: String, url: String) {
    let data_dir = dirs::data_dir().unwrap().join("didyoumean");

    if !data_dir.is_dir() {
        std::fs::create_dir(data_dir).expect("Failed to create data directory");
    }

    let file_path = dirs::data_dir()
        .unwrap()
        .join("didyoumean")
        .join(lang.to_owned());

    if !file_path.is_file() {
        println!(
            "Downloading word list for {}...",
            lang.to_string().blue()
        );

        // Setup Reqwest
        let response = reqwest::get(url.to_owned()).await.expect("Request failed");
        let total_size = response.content_length().unwrap();

        // Setup indicatif
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.blue/cyan}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-")
        );

        // Setup file and stream.
        let mut file = std::fs::File::create(file_path).expect("Failed to create file");
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        // Read from stream into file.
        while let Some(item) = stream.next().await {
            let chunk = item.expect("Error downloading file");
            file.write_all(&chunk).expect("Error while writing to file");
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        // Print completed bar.
        pb.finish_at_current_pos();
    }
}
