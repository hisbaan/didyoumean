use cli_clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use std::cmp::min;

#[cfg(unix)]
use nix::unistd::{fork, ForkResult};

/// Copy `string` to the system clipboard
///
/// # Arguments
///
/// * `string` - the string to be copied.
pub fn yank(string: &str) {
    let platform = std::env::consts::OS;
    if vec![
        "linux",
        "freebsd",
        "netbsd",
        "dragonfly",
        "mac osx",
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
                    let clipboard = ctx.get_contents().unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
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

/// Insert `element` at `index` preserving length.
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
/// # use didyoumean::insert_and_shift;
/// let mut to_shift = vec![0, 1, 2, 3, 4];
/// insert_and_shift(&mut to_shift, 2, 11);
///
/// assert_eq!(to_shift, vec![0, 1, 11, 2, 3]);
/// ```
pub fn insert_and_shift<T: Copy>(list: &mut Vec<T>, index: usize, element: T) {
    if index > list.len() - 1 {
        return;
    }

    list.insert(index, element);
    list.truncate(list.len() - 1);
}

/// Return the edit distance between `search_term` and `known_term`.
/// Currently implemented using a modified version of
/// [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance).
///
/// # Arguments
///
/// * `search_chars` - The first `Vec<char>` to compare, in most time search_term will not change, so
/// we would like to share the same `Vec<char>` between multiple calls. you could use `search_string.chars().collect::<Vec<_>>()` to
/// convert a string to a `Vec<char>`
/// * `known_term` - The second string to compare
///
/// # Examples
///
/// ```
/// # use didyoumean::edit_distance;
/// let dist = edit_distance(&"sitting".chars().collect::<Vec<_>>(), "kitten");
/// assert_eq!(dist, 3);
/// assert_eq!(edit_distance(&"geek".chars().collect::<Vec<_>>(), "gesek"), 1);
/// assert_eq!(edit_distance(&"cat".chars().collect::<Vec<_>>(), "cut"), 1);
/// assert_eq!(edit_distance(&"sunday".chars().collect::<Vec<_>>(), "saturday"), 3);
/// assert_eq!(edit_distance(&"tset".chars().collect::<Vec<_>>(), "test"), 1);
/// ```
#[allow(clippy::iter_count, clippy::needless_range_loop)]
pub fn edit_distance(search_chars: &[char], known_term: &str) -> usize {
    // Set local constants for repeated use later.
    let known_chars: Vec<char> = known_term.chars().collect();
    let n = search_chars.iter().count() + 1;
    let m = known_chars.iter().count() + 1;

    // Setup matrix 2D vector.
    let mut mat = vec![0; m * n];

    // Initialize values of the matrix.
    for i in 1..n {
        mat[i * m] = i;
    }
    for i in 1..m {
        mat[i] = i;
    }

    // Run the algorithm.
    for i in 1..n {
        // let search_char_i_minus_one = search_chars[i - 1];
        // let search_char_i_minus_two = if i > 1 { search_chars[i - 2] } else { ' ' };
        for j in 1..m {
            let sub_cost = if search_chars[i - 1] == known_chars[j - 1] {
                0
            } else {
                1
            };

            mat[i * m + j] = min(
                mat[(i - 1) * m + j - 1] + sub_cost, // substitution cost
                min(
                    mat[(i - 1) * m + j] + 1, // deletion cost
                    mat[i * m + j - 1] + 1,   // insertion cost
                ),
            );
            if i > 1
                && j > 1
                && search_chars[i - 1] == known_chars[j - 2]
                && search_chars[i - 2] == known_chars[j - 1]
            {
                mat[i * m + j] = min(
                    mat[i * m + j],
                    mat[(i - 2) * m + j - 2] + 1, // transposition cost
                );
            }
        }
    }

    // Return the bottom left corner of the matrix.
    mat[m * n - 1]
}
