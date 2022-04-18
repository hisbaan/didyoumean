pub use std::cmp::{min, Ordering};
pub use cli_clipboard::{ClipboardContext, ClipboardProvider};
pub use colored::*;

#[cfg(unix)]
pub use nix::unistd::{fork, ForkResult};

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
                    let clipboard = format!("{}", ctx.get_contents().unwrap());
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
/// # use didyoumean::insert_and_shift;
/// let to_shift = vec![0, 1, 2, 3, 4];
/// let shifted = insert_and_shift(to_shift, 2, 11);
/// assert_eq!(shifted, vec![0, 1, 11, 2, 3]);
/// ```
pub fn insert_and_shift<T: Copy>(list: Vec<T>, index: usize, element: T) -> Vec<T> {
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
/// # use didyoumean::edit_distance;
/// let dist = edit_distance("sitting", "kitten");
/// assert_eq!(dist, 3)
/// ```
pub fn edit_distance(search_term: &str, known_term: &str) -> usize {
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
