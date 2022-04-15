use std::cmp::min;
use clap::Parser;

const WORDS: &str = include_str!("words.txt");

// Parse command line arguments to get the search term.
#[derive(Parser)]
#[clap(author = "Hisbaan Noorani", version = "0.1.0", about = "Did You Mean: A cli spell checking program", long_about = None)]
struct Cli {
    search_term: String,
    #[clap(short = 'n', long = "name", default_value_t = 5)]
    number: usize,
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() {
    // Parse args using clap.
    let args = Cli::parse();

    // Get dictionary of words from words.txt.
    let dictionary = WORDS.split('\n');

    // Create mutable vecs for storing the top n words.
    let mut top_n_words = vec!["" ; args.number];
    let mut top_n_dists = vec![args.search_term.len() * 10 ; args.number];

    // Loop over the words in the dictionary, run the algorithm, and
    // add to the list if appropriate
    for word in dictionary {
        // Get edit distance.
        let dist = edit_distance(&args.search_term, word);

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
    for i in 0..args.number {
        if args.verbose { print!("{}, ", top_n_dists[i]); }
        println!("{}", top_n_words[i]);
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
    if index > list.len() - 1 { return list; }

    let mut temp = list.clone();

    for i in 0..list.len() {
        if i == index {
            temp[i] = element;

        } else if i > index {
            temp[i] = list[i - 1];
        } else {
            temp[i] = list[i];
        }
    }

    return temp;
}

/// Return the edit distance between `search_term` and `known_term`. Currently implemented
/// using [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance).
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
    let mut mat = vec![vec![0 ; m] ; n];

    // Initialize values of the matrix.
    for i in 1..n { mat[i][0] = i; }
    for i in 1..m { mat[0][i] = i; }

    // Run the algorithm.
    for i in 1..n {
        for j in 1..m {
            let mut sub_cost = 1;
            if search_chars[i - 1] == known_chars[j - 1] {
                sub_cost = 0;
            }

            mat[i][j] = min(
                mat[i - 1][j - 1] + sub_cost,   // substitution cost
                min(
                    mat[i - 1][j] + 1,          // deletion cost
                    mat[i][j - 1] + 1,          // insertion cost
                ));
        }
    }

    // Return the bottom left corner of the matrix.
    return mat[n-1][m-1];
}
