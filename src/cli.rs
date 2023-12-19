use clap::Parser;

// Parse command line arguments to get the search term.
#[derive(Parser)]
#[clap(author = "Hisbaan Noorani", version = "1.1.4", about = "Did You Mean: A cli spelling corrector", long_about = None)]
pub struct Cli {
    pub search_term: Option<String>,
    #[clap(
        short = 'n',
        long = "number",
        default_value_t = 5,
        help = "Change the number of matches printed",
        long_help = "Change the number of words the program will print. The default value is five."
    )]
    pub number: usize,
    #[clap(
        short = 'c',
        long = "clean-output",
        help = "Print clean output",
        long_help = "Print a clean version of the output without the title, numbers or colour."
    )]
    pub clean_output: bool,
    #[clap(
        short = 'v',
        long = "verbose",
        help = "Print verbose output",
        long_help = "Print verbose output including the edit distance of the found word to the queried word."
    )]
    pub verbose: bool,
    #[clap(
        short = 'y',
        long = "yank",
        help = "Yank (copy) to the system cliboard",
        long_help = "Yank (copy) the selected word to the system clipboard. If no word is selected, the clipboard will not be altered."
    )]
    pub yank: bool,
    #[clap(
        short = 'l',
        long = "lang",
        help = "Select the desired language using the locale code (en, fr, sp, etc.)",
        long_help = "Select the desired language using its locale code. For example, English would have the locale code en and French would have the locale code fr. See --print-langs for a list of locale codes and the corresponding languages.",
        default_value = "en"
    )]
    pub lang: String,
    #[clap(
        long = "print-langs",
        help = "Display a list of supported languages",
        long_help = "Display a list of supported languages and their respective locale codes."
    )]
    pub print_langs: bool,
    #[clap(
        long = "update-langs",
        help = "Update all language files",
        long_help = "Update all language files from the repository specified by CLI @wordlist-url@."
    )]
    pub update_langs: bool,
    #[clap(
        short = 'w',
        long = "wordlist-url",
        help = "Wordlist repository URL",
        long_help = "Wordlist repository URL. The default value is 'https://raw.githubusercontent.com/hisbaan/wordlists/main'",
        default_value = "https://raw.githubusercontent.com/hisbaan/wordlists/main"
    )]
    pub wordlist_url: String,
}
