use std::cmp::min;
use clap::Parser;

const WORDS: &str = include_str!("words.txt");

// Parse command line arguments to get the search term.
#[derive(Parser)]
struct Cli {
    search_term: String,
}

fn main() {
    let args = Cli::parse();

    // println!("Loading dictionary...");
    let dictionary = WORDS.split('\n');
    // println!("Done loading dictionary...");

    let mut top_n_w = vec!["" ; 5];
    let mut top_n_d = vec![args.search_term.len() * 10 ; 5];

    // println!("Running algorithm...");
    for word in dictionary {
        let dist = edit_distance(&args.search_term, word);

        if dist < top_n_d[4] {
            for i in 0..5 {
                if dist < top_n_d[i] {
                    top_n_d = insert_and_shift(top_n_d, i, dist);
                    top_n_w = insert_and_shift(top_n_w, i, word);
                    break;
                }
            }
        }
    }

    for i in 0..5 {
        println!("{}, {}", top_n_w[i], top_n_d[i]);
    }
}

fn insert_and_shift<T: Copy>(list: Vec<T>, index: usize, element: T) -> Vec<T> {
    if index > list.len() - 1 { return list; }

    let mut temp = list.clone();

    for i in 0..list.len() {
        if i == index {
            temp[i] = element;
        } else {
            temp[i] = list[i];
        }
    }

    return temp;
}

fn edit_distance(search_term: &str, known_term: &str) -> usize {
    let n = search_term.len() + 1;
    let m = known_term.len() + 1;
    let search_chars: Vec<char> = search_term.chars().collect();
    let known_chars: Vec<char> = known_term.chars().collect();

    let mut mat = vec![vec![0 ; m] ; n];

    for i in 1..n {
        mat[i][0] = i;
    }

    for i in 1..m {
        mat[0][i] = i;
    }

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

    // for i in 0..n {
    //     for j in 0..m {
    //         print!{"{} ", mat[i][j]}
    //     }
    //     println!("");
    // }

    return mat[n-1][m-1];
}
