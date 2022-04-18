use didyoumean::yank;
use cli_clipboard::{ClipboardProvider, ClipboardContext};

#[test]
fn yank_test() {
    let string = "test";
    let not_string = "not test";
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // Run the yank function.
    yank(string);

    // Sleep to allow the function time to write to the clipboard.
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Get the clipboard contents.
    let clipboard = format!("{}", ctx.get_contents().unwrap());

    assert_eq!(clipboard, string);

    // Set the clipboard contents to something else to get the process to exit.
    ctx.set_contents(not_string.to_owned()).unwrap();
}
