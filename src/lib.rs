pub use cli_clipboard::{ClipboardContext, ClipboardProvider};
pub use colored::*;
pub use std::cmp::min;

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
/// * `search_term` - The first string to compare
/// * `known_term` - The second string to compare
///
/// # Examples
///
/// ```
/// # use didyoumean::edit_distance;
/// let dist = edit_distance("sitting", "kitten");
/// assert_eq!(dist, 3);
/// assert_eq!(edit_distance("geek", "gesek"), 1);
/// assert_eq!(edit_distance("cat", "cut"), 1);
/// assert_eq!(edit_distance("sunday", "saturday"), 3);
/// ```
pub fn edit_distance(search_term: &str, known_term: &str) -> usize {
    // Set local constants for repeated use later.
    let n = search_term.len() + 1;
    let m = known_term.len() + 1;
    let mut search_chars = search_term.chars();

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
        let mut known_chars = known_term.chars();
        let search_char = search_chars.next().unwrap();
        for j in 1..m {
            let sub_cost = if search_char == known_chars.next().unwrap() {
                0
            } else {
                1
            };

            mat[i][j] = min(
                mat[i - 1][j - 1] + sub_cost, // substitution cost
                min(
                    mat[i - 1][j] + 1, // deletion cost
                    mat[i][j - 1] + 1, // insertion cost
                ),
            );
            // if i > 1
            //     && j > 1
            //     && search_chars[i - 1] == known_chars[j - 2]
            //     && search_chars[i - 2] == known_chars[j - 1]
            // {
            //     mat[i][j] = min(
            //         mat[i][j],
            //         mat[i - 2][j - 2] + 1, // transposition cost
            //     );
            // }
        }
    }

    // Return the bottom left corner of the matrix.
    mat[n - 1][m - 1]
}
